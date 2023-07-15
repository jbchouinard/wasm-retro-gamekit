use std::sync::atomic;

/// ObjectId is a globally unique identifier for objects in the physics system.
#[derive(PartialEq, Eq)]
pub struct ObjectId(pub(super) u64);

/// ObjectKey is the externally copyable version of ObjectId.
/// It can be copied and used to index objects, but not to identify them.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ObjectKey(pub(super) u64);

pub trait Identity {
    fn id(&self) -> &ObjectId;
}

pub trait IdentityKey {
    fn key(&self) -> ObjectKey;
}

impl<T> IdentityKey for T
where
    T: Identity,
{
    fn key(&self) -> ObjectKey {
        ObjectKey(self.id().0)
    }
}

static OBJECT_ID_COUNTER: atomic::AtomicU64 = atomic::AtomicU64::new(0);

impl ObjectId {
    pub fn new() -> Self {
        let n = OBJECT_ID_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
        assert_ne!(n, u64::MAX, "allocated u64::MAX ObjectIds!?");
        Self(n)
    }
}
