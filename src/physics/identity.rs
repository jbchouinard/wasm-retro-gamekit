use std::{cmp::Ordering, sync::atomic};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct ObjectId(u64);

static OBJECT_ID_COUNTER: atomic::AtomicU64 = atomic::AtomicU64::new(0);

impl ObjectId {
    pub fn new() -> Self {
        let n = OBJECT_ID_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
        assert_ne!(n, u64::MAX, "allocated u64::MAX ObjectIds!?");
        Self(n)
    }
}

impl ObjectId {
    // ObjectId does not implement Clone or Copy by design, it is intended as
    // a unique, opaque handle. Private copy method to put it in HashMaps and such.
    pub(super) fn copy(&self) -> Self {
        Self(self.0)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub(super) struct ObjectIdPair(pub ObjectId, pub ObjectId);

impl ObjectIdPair {
    pub(super) fn new(oid1: &ObjectId, oid2: &ObjectId) -> Self {
        match oid1.cmp(oid2) {
            Ordering::Greater => ObjectIdPair(oid1.copy(), oid2.copy()),
            Ordering::Less => ObjectIdPair(oid2.copy(), oid1.copy()),
            Ordering::Equal => panic!("got duplicate {:?}", oid1),
        }
    }
    pub(super) fn copy(&self) -> Self {
        ObjectIdPair(self.0.copy(), self.1.copy())
    }
}
