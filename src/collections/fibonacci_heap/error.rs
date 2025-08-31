use std::cell::BorrowMutError;

pub enum HeapReferenceError {
    RecursiveExclusiveAccess,
}

impl From<BorrowMutError> for HeapReferenceError {
    fn from(_: BorrowMutError) -> Self {
        Self::RecursiveExclusiveAccess
    }
}
