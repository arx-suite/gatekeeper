#![cfg_attr(not(feature = "std"), no_std)]

// `rc` feature: Rc/Arc support is not implemented yet.
#[cfg(feature = "rc")]
compile_error!(
    "Gatekeeper: unsupported feature `rc` enabled\n\n\
    The `rc` feature enables validations for `Rc<T>` and `Arc<T>` wrappers.\n\n\
    Reason: Shared-pointer handling (especially for transformations and in-place \
    mutation) requires careful API design and strong invariants. In this initial \
    release, Rc/Arc support is intentionally disabled to avoid surprising \
    behavior and complexity.\n\n\
    Consequences:\n\
    - Validation and transform semantics for shared pointers are undefined.\n\
    - Transform operations that require ownership will not work with Rc/Arc.\n\n\
    What to do:\n\
    - Disable `rc` in your Cargo features for now.\n\
    - If you need shared-pointer validation today, validate the inner value before wrapping it.\n\
    - We plan to add rc/arc support in a future release; track the roadmap or open an issue.\n
    "
);

// `sanitize` feature: sanitization pipeline is not implemented yet.
#[cfg(feature = "sanitize")]
compile_error!(
    "Gatekeeper: unsupported feature `sanitize` enabled\n\n\
    The `sanitize` feature intends to provide HTML/script stripping, trimming, \
    normalization, and other safety-focused transformations as a first-class \
    stage in the pipeline.\n\n\
    Reason: Sanitization semantics (what gets removed or escaped) are domain-specific\n\
    and require a stable API, careful security review, and bundled implementations.\n    \
    We have not stabilized that API in the initial release.\n\n\
    Consequences:\n\
    - No sanitizer functions or derive integration will be available.\n\
    - Enabling this feature will not provide secure sanitization and will fail compilation.\n\n\
    What to do:\n\
    - Disable `sanitize` for now, and perform sanitization in your application code as needed.\n\
    - Follow the repository for upcoming sanitizer implementation details and examples.\n
    "
);

// `transform` feature: transformation pipeline is not implemented yet.
#[cfg(feature = "transform")]
compile_error!(
    "Gatekeeper: unsupported feature `transform` enabled\n\n\
    The `transform' feature would add in-place or ownership-based transformations\n\
    (e.g. trim, lowercase, normalize) integrated with derive macros.\n\n\
    Reason: Transform semantics interact closely with ownership and smart-pointer\n\
    behavior (e.g. Arc/Rc). We intentionally omitted this from the first release\n\
    to avoid unsafe or surprising behavior.\n\n\
    Consequences:\n\
    - No built-in transform functions are available with this feature enabled.\n\
    - The derive macro will not emit transform code; compilation will fail.\n\n\
    What to do:\n\
    - Disable `transform` for now and apply transformations explicitly in your codebase.\n\
    - Check the roadmap for planned transform API and examples.\n
    "
);

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

pub mod error;
pub mod validate;

pub use error::Report;
