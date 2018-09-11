use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

/// Types that are safe to use with StackVec.
/// It's very very unsafe to implement this for some custom type and then use it.
/// Just don't.
pub unsafe trait Array {
    type Item;
    fn size() -> usize;
}
macro_rules! impl_array {
    ($($len:expr),*) => {
        $(unsafe impl<T> Array for [T; $len] {
            type Item = T;

            fn size() -> usize {
                $len
            }
        })*
    }
}
impl_array!(1, 2, 3, 4, 5, 6, 7, 8);

/// A stack allocated vector-like container
pub struct StackVec<T: Array> {
    array: T,
    len: usize
}
impl<T: Array> Default for StackVec<T> {
    fn default() -> Self {
        Self {
            array: unsafe { mem::uninitialized() },
            len: 0
        }
    }
}
impl<T: Array> StackVec<T> {
    /// Create a new instance
    pub fn new() -> Self {
        Self::default()
    }
    /// Return the length of this container
    pub fn len(&self) -> usize {
        self.len
    }

    fn ptr(&self) -> *const T::Item {
        &self.array as *const _ as *const T::Item
    }
    fn ptr_mut(&mut self) -> *mut T::Item {
        &mut self.array as *mut _ as *mut T::Item
    }

    /// Push a new item
    pub fn push(&mut self, item: T::Item) {
        assert!(self.len < T::size(), "stackvec::push called on an already filled array");
        assert!(self.len <= std::isize::MAX as usize);

        unsafe {
            ptr::write(
                self.ptr_mut().offset(self.len as isize),
                item
            );
        }
        self.len += 1;
    }
    /// Push multiple new items at once
    pub fn append<Q>(&mut self, items: Q)
        where Q: Array<Item = T::Item>
    {
        assert!(self.len + Q::size() <= T::size(), "stackvec::append called with a larger array than capacity");
        assert!(self.len <= std::isize::MAX as usize);

        unsafe {
            ptr::copy(
                &items as *const _ as *const T::Item,
                self.ptr_mut().offset(self.len as isize),
                Q::size()
            );
        }
        self.len += Q::size();
    }
}
impl<T: Array> Deref for StackVec<T> {
    type Target = [T::Item];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}
impl<T: Array> DerefMut for StackVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr_mut(), self.len)
        }
    }
}
impl<'a, T: Array> IntoIterator for &'a StackVec<T> {
    type Item = &'a T::Item;
    type IntoIter = std::slice::Iter<'a, T::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}