#[derive(Debug)]
pub enum HeapReferenceError {
    RecursiveExclusiveHeapAccess,
    ElementRemoved,
}
