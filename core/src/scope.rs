use core::num::NonZero;

type ScopeInt = u32;
const MAX_SCOPES: usize = ScopeInt::BITS as usize;

pub struct ScopeHandle {
    /// The bits set in the bitmask correspond to this scope,
    /// and all the (transitive) subscopes of this scope.
    bitmask: NonZero<ScopeInt>,
}

impl ScopeHandle {}
