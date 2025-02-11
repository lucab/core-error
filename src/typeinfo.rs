use core::any::{Any, TypeId};

/// Unsafe trait to get the type_id of 'static types. Implemented for all types, so it can be
/// easily added as a supertrait.
pub unsafe trait TypeInfo {
    /// Gets the `TypeId` of `self`.

    // The bound is slightly different from the RFC in order to make it work on
    // as many rust versions as possible.
    //
    // Any is functionally equivalent to 'static, but on Rust 1.6.0,
    // `TypeId::of` requires the marker trait Reflect. This trait is implemented
    // on all types that impl Any, and Any is impl'd for every type where T:
    // 'static.
    fn type_id(&self) -> TypeId
    where
        Self: Any,
    {
        TypeId::of::<Self>()
    }
}

unsafe impl<T: ?Sized> TypeInfo for T {}
