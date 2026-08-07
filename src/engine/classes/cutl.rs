use std::marker::PhantomData;

#[repr(C)]
pub struct CUtlVector<T> {
  // memory: CAllocator,
  size: i32,
  _t: PhantomData<T>
}

#[repr(C)]
pub struct CUtlString;