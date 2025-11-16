//! Error types used by `garde`.
//!
//! The entrypoint of this module is the [`Error`] type.

mod rc_list;

use compact_str::{CompactString, ToCompactString};
use rc_list::List;
use smallvec::SmallVec;

const DEFAULT_MAX_ERROR_REPORT: usize = 10;

/// A validation error report.
///
/// This type is used as a container for errors aggregated during validation.
/// It is a flat list of `(Path, Error)`.
/// A single field or list item may have any number of errors attached to it.
#[derive(Clone, Debug)]
pub struct Report {
    errors: SmallVec<[(Path, Error); DEFAULT_MAX_ERROR_REPORT]>,
}

impl Report {
    /// Create an empty [`Report`].
    pub fn new() -> Self {
        Self {
            errors: SmallVec::new(),
        }
    }
    /// Append an [`Error`] into this report at the given [`Path`].
    pub fn append(&mut self, path: Path, error: Error) {
        self.errors.push((path, error));
    }

    /// Iterate over all `(Path, Error)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(Path, Error)> {
        self.errors.iter()
    }

    /// Returns `true` if the report contains no validation errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Converts into the inner validation errors.
    pub fn into_inner(self) -> SmallVec<[(Path, Error); DEFAULT_MAX_ERROR_REPORT]> {
        self.errors
    }
}

impl core::fmt::Display for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (path, error) in self.iter() {
            if path.is_empty() {
                writeln!(f, "{error}")?;
            } else {
                writeln!(f, "{path}: {error}")?;
            }
        }
        Ok(())
    }
}

impl core::error::Error for Report {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    message: CompactString,
}

impl Error {
    pub fn new(message: impl ToCompactString) -> Self {
        Self {
            message: message.to_compact_string(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    components: List<(Kind, CompactString)>,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    None,
    Key,
    Index,
}

/// Represents a path component without a key. This is useful when the container
/// only ever holds a single key, which is the case for any 1-tuple struct.
#[derive(Default)]
pub struct NoKey(());

impl core::fmt::Display for NoKey {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

pub trait PathComponentKind: core::fmt::Display + ToCompactString {
    fn component_kind() -> Kind;
}

macro_rules! impl_path_component_kind {
    ($(@$($G:lifetime)*;)? $T:ty => $which:ident) => {
        impl $(<$($G),*>)? PathComponentKind for $T {
            fn component_kind() -> Kind {
                Kind::$which
            }
        }
    }
}

// TODO: need to implement for heap allocating structures as well
impl_path_component_kind!(usize => Index);
impl_path_component_kind!(@'a; &'a str => Key);
impl_path_component_kind!(CompactString => Key);
impl_path_component_kind!(NoKey => None);

impl<T: PathComponentKind> PathComponentKind for &T {
    fn component_kind() -> Kind {
        T::component_kind()
    }
}

impl Path {
    pub fn empty() -> Self {
        Self {
            components: List::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn new<C: PathComponentKind>(component: C) -> Self {
        Self {
            components: List::new().append((C::component_kind(), component.to_compact_string())),
        }
    }

    pub fn join<C: PathComponentKind>(&self, component: C) -> Self {
        Self {
            components: self
                .components
                .append((C::component_kind(), component.to_compact_string())),
        }
    }

    #[doc(hidden)]
    pub fn __iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Kind, &CompactString)> + ExactSizeIterator {
        let mut components = TempComponents::with_capacity(self.components.len());
        for (kind, component) in self.components.iter() {
            components.push((*kind, component));
        }
        components.into_iter()
    }
}

type TempComponents<'a> = SmallVec<[(Kind, &'a CompactString); 8]>;

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        struct Components<'a> {
            path: &'a Path,
        }

        impl core::fmt::Debug for Components<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut list = f.debug_list();
                list.entries(self.path.__iter().rev().map(|(_, c)| c))
                    .finish()
            }
        }

        f.debug_struct("Path")
            .field("components", &Components { path: self })
            .finish()
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut components = self.__iter().rev().peekable();
        let mut first = true;
        while let Some((kind, component)) = components.next() {
            if first && kind == Kind::Index {
                f.write_str("[")?;
            }
            first = false;
            f.write_str(component.as_str())?;
            if kind == Kind::Index {
                f.write_str("]")?;
            }
            if let Some((kind, _)) = components.peek() {
                match kind {
                    Kind::None => {}
                    Kind::Key => f.write_str(".")?,
                    Kind::Index => f.write_str("[")?,
                }
            }
        }

        Ok(())
    }
}
