//! A Rust port of the C macro [`container_of`].
//!
//! This macro is used to convert from a pointer to a struct's field to a pointer to the struct
//! itself.
//! Note that the struct should be sized.
//!
//! # Example
//! ```rust
//! #[repr(C)]
//! struct ListNode<T> {
//! 	prev: Option<Box<ListNode<T>>>,
//! 	next: Option<Box<ListNode<T>>>,
//! 	data: T
//! }
//!
//! # fn main() {
//! let list = ListNode { prev: None, next: None, data: 123 };
//!
//! // Get a pointer to the `data` from `list`.
//! let data_ptr = &list.data as *const i32;
//! 
//! // Get the container of `data_ptr`, ie the `ListNode` it was made within.
//! // SAFETY: `data_ptr` is a valid pointer to the `data` field of a
//! // `ListNode<i32>`. Additionally, `ListNode<i32>` is sized.
//! let list_ptr = unsafe {
//! 	container_of::container_of!(data_ptr, ListNode<i32>, data)
//! };
//!
//! // The resulting pointer is the same as if you just got it straight
//! // from the containing structure.
//! assert_eq!(list_ptr, &list as *const ListNode<i32>);
//! # }
//! ```
//! 
//! # Safety
//! The following are needed to ensure soundness:
//! - The `$type` must be a sized struct that is `#[repr(C)]` (or `#[repr(packed)]`).
//! - The `$ptr` must be a valid pointer to the `$field` field of a `$type`. More concretely, this
//!   means that the `$ptr` must have originated from a valid `$type` struct.
//!
//! [`container_of`]: https://github.com/torvalds/linux/blob/f71077a4d84bbe8c7b91b7db7c4ef815755ac5e3/tools/include/linux/kernel.h#L33-L35

pub use memoffset::offset_of;

/// The [`container_of`] macro.
///
/// See the crate-level docs for more info and safety considerations.
///
/// [`container_of`]: https://github.com/torvalds/linux/blob/f71077a4d84bbe8c7b91b7db7c4ef815755ac5e3/tools/include/linux/kernel.h#L33-L35
#[macro_export]
macro_rules! container_of {
	($ptr:expr, $type:path, $field:ident) => {
		$ptr.cast::<u8>()
			.sub($crate::offset_of!($type, $field))
			.cast::<$type>()
	};
}

#[cfg(test)]
mod tests {
	#[allow(unused)]
	#[repr(C)]
	struct Wrapper<T: ?Sized> {
		foo: i32,
		bar: u8,
		inner: T
	}

	#[test]
	fn works_with_both_mutable_and_const_pointers() {
		let mut wrap = Wrapper { foo: 1234, bar: 56, inner: 78u8 };

		let inner_ptr = &wrap.inner as *const u8;
		let _: *const Wrapper<u8> = unsafe {
			crate::container_of!(inner_ptr, Wrapper<u8>, inner)
		};

		let inner_ptr_mut = &mut wrap.inner as *mut u8;
		let _: *mut Wrapper<u8> = unsafe {
			crate::container_of!(inner_ptr_mut, Wrapper<u8>, inner)
		};
	}

	#[test]
	fn no_padding() {
		let wrap = Wrapper { foo: 1234, bar: 56, inner: 78u8 };

		let inner_ptr = &wrap.inner as *const u8;
		let wrap_ptr = unsafe {
			crate::container_of!(inner_ptr, Wrapper<u8>, inner)
		};

		assert_eq!(&wrap as *const Wrapper<u8>, wrap_ptr);
	}

	#[test]
	fn with_padding() {
		let wrap = Wrapper { foo: 1234, bar: 56, inner: 78i32 };

		let inner_ptr = &wrap.inner as *const i32;
		let wrap_ptr = unsafe {
			crate::container_of!(inner_ptr, Wrapper<i32>, inner)
		};

		assert_eq!(&wrap as *const Wrapper<i32>, wrap_ptr);
	}
}
