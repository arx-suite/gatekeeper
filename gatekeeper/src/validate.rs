//! ## Core validation traits and types

use core::fmt::Debug;

use crate::Report;
use crate::error::Path;

/// The core trait of this crate.
///
/// Validation runs the fields through every validation rules,
/// and aggregates any errors into a [`Report`].
pub trait Validate {
    /// A user-provided context.
    ///
    /// Custom validators receive a reference to this context.
    type Context;

    /// Validates `Self`, returning an `Err` with an aggregate of all errors if
    /// the validation failed.
    ///
    /// This method should not be implemented manually. Implement [`Validate::validate_into`] instead,
    /// because [`Validate::validate`] has a default implementation that calls [`Validate::validate_into`].
    fn validate(&self) -> Result<(), Report>
    where
        Self::Context: Default,
    {
        let ctx = Self::Context::default();
        self.validate_with(&ctx)
    }

    /// Validates `Self`, returning an `Err` with an aggregate of all errors if
    /// the validation failed.
    ///
    /// This method should not be implemented manually. Implement [`Validate::validate_into`] instead,
    /// because [`Validate::validate_with`] has a default implementation that calls [`Validate::validate_into`].
    fn validate_with(&self, ctx: &Self::Context) -> Result<(), Report> {
        let mut report = Report::new();
        self.validate_into(ctx, &mut Path::empty, &mut report);
        match report.is_empty() {
            true => Ok(()),
            false => Err(report),
        }
    }

    /// Validates `Self`, aggregating all validation errors into `Report`.
    fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut dyn FnMut() -> Path,
        report: &mut Report,
    );
}

/// A struct which wraps a valid instance of some `T`.
///
/// The only way to create an instance of this struct is through the `validate`
/// function on the [`Unvalidated`] type. This ensures that if you have a `Valid<T>`,
/// it was definitely validated at some point. This is commonly referred to as the
/// typestate pattern.
#[derive(Debug, Clone, Copy)]
pub struct Valid<T>(T);

impl<T: Validate> Valid<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A struct which wraps a potentially invalid instance of some `T`.
///
/// Use the `validate` method to turn this type into a `Valid<T>`.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Unvalidated<T>(T);

impl<T: Validate> Unvalidated<T> {
    /// Creates an `Unvalidated<T>`
    pub fn new(v: T) -> Self {
        Self(v)
    }

    /// Validates `self`, transforming it into a `Valid<T>`.
    /// This is the only way to create an instance of `Valid<T>`.
    pub fn validate(self) -> Result<Valid<T>, Report>
    where
        <T as Validate>::Context: Default,
    {
        self.0.validate()?;
        Ok(Valid(self.0))
    }

    /// Validates `self`, transforming it into a `Valid<T>`.
    /// This is the only way to create an instance of `Valid<T>`.
    pub fn validate_with(self, ctx: &<T as Validate>::Context) -> Result<Valid<T>, Report> {
        self.0.validate_with(ctx)?;
        Ok(Valid(self.0))
    }
}

impl<T: Validate> From<T> for Unvalidated<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Debug> Debug for Unvalidated<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: ?Sized + Validate> Validate for &T {
    type Context = T::Context;

    fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut dyn FnMut() -> Path,
        report: &mut Report,
    ) {
        <T as Validate>::validate_into(self, ctx, parent, report)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: ?Sized + Validate> Validate for &mut T {
    type Context = T::Context;

    fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut dyn FnMut() -> Path,
        report: &mut Report,
    ) {
        <T as Validate>::validate_into(self, ctx, parent, report)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;

    struct User {
        pub username: String,
        pub age: u8,
    }

    impl Validate for User {
        type Context = ();

        fn validate_into(
            &self,
            _: &Self::Context,
            parent: &mut dyn FnMut() -> Path,
            report: &mut Report,
        ) {
            if self.username.trim().is_empty() {
                let path = parent().join("username");
                report.append(path, Error::new("username must not be empty"));
            }

            if self.age < 18 {
                let path = parent().join("age");
                report.append(path, Error::new("age must be ≥ 18"));
            }
        }
    }

    #[test]
    fn valid_user_passes() {
        let user = User {
            username: "bob".into(),
            age: 22,
        };

        let result = user.validate();
        assert!(result.is_ok(), "Expected validation to pass");
    }
}
