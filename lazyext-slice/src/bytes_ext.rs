#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::mem;
use core::ptr::slice_from_raw_parts;
use core::slice::{from_raw_parts, from_raw_parts_mut};

macro_rules! cfg_bytes {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "bytes")]
            #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
            $item
        )*
    }
}

#[cfg(feature = "alloc")]
macro_rules! to_x_vec_impl {
    ($this:ident, $typ:tt::$conv:tt) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        let src = $this.as_bytes_ref();
        assert_eq!(
            src.len() % SIZE,
            0,
            "invalid length of u8 slice: {}",
            src.len()
        );
        let ptr = src.as_ptr();
        (0..src.len())
            .step_by(SIZE)
            .map(|v| unsafe { $typ::$conv(*(ptr.add(v) as *const _ as *const [_; SIZE])) })
            .collect::<Vec<_>>()
    }};
}

macro_rules! to_x_slice_impl_suite_in {
    ($this: ident, $builder:ident, $trait:tt::$fn:tt::$ptr:tt, $raw_ptr: ident, $ty: ty) => {{
        let src = $trait::$fn($this);
        assert_eq!(src.len() % mem::size_of::<$ty>(), 0, "invalid length of u8 slice: {}", src.len());
        let ptr = src.$ptr() as *const $ty;
        unsafe {
            $builder(ptr as *$raw_ptr $ty, src.len() / mem::size_of::<$ty>())
        }
    }};
}

macro_rules! to_x_slice_impl_suite {
    ($builder:ident, $trait:tt::$fn:tt::$ptr:tt, $raw_ptr: ident, $([$ty: ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Convert u8 slice to ", $ty_literal, " slice in native-endian(zero-copy)")]
            fn [<to_ $ty _slice>](&self) -> &[$ty] {
                to_x_slice_impl_suite_in!(self, $builder, $trait::$fn::$ptr, $raw_ptr, $ty)
            }
        }
        )*
    };
    (mut $builder:ident, $trait:tt::$fn:tt::$ptr:tt, $raw_ptr: ident, $([$ty: ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Convert mutable u8 slice to mutable", $ty_literal, " slice in native-endian(zero-copy)")]
            fn [<to_ $ty _slice_mut>](&mut self) -> &[$ty] {
                to_x_slice_impl_suite_in!(self, $builder, $trait::$fn::$ptr, $raw_ptr, $ty)
            }
        }
        )*
    };
}

#[cfg(feature = "alloc")]
macro_rules! to_x_slice_lossy_impl {
    ($this:ident, $typ: ident) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        let src = $this.as_bytes_ref();
        assert_eq!(
            src.len() % SIZE,
            0,
            "invalid length of u8 slice: {}",
            src.len()
        );
        let ptr = src.as_ptr() as *const $typ;
        let lossy = unsafe { &*slice_from_raw_parts(ptr, src.len() / SIZE) };
        Cow::Borrowed(lossy)
    }};
}

#[cfg(feature = "alloc")]
macro_rules! to_x_vec_impl_suite {
    ($([$ty:ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Copy u8 slice to ", $ty_literal, " vec in big-endian")]
            #[inline]
            fn [<to_be_ $ty _vec>](&self) -> Vec<$ty> {
                to_x_vec_impl!(self, $ty::from_be_bytes)
            }

            #[doc = concat!("Copy u8 slice to ", $ty_literal, " vec in little-endian")]
            #[inline]
            fn [<to_le_ $ty _vec>](&self) -> Vec<$ty> {
                to_x_vec_impl!(self, $ty::from_le_bytes)
            }

            #[doc = concat!("Copy u8 slice to ", $ty_literal, " vec in native-endian")]
            #[inline]
            fn [<to_ne_ $ty _vec>](&self) -> Vec<$ty> {
                to_x_vec_impl!(self, $ty::from_ne_bytes)
            }
        }
        )*
    };
}

#[cfg(feature = "alloc")]
macro_rules! to_x_slice_lossy_impl_suite {
    ($([$ty: ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("convert u8 slice to Cow<'_, [", $ty_literal, "]> in native-endian (zero-copy)")]
            fn [<to_ $ty _slice_lossy>](&self) -> Cow<'_, [$ty]> {
                to_x_slice_lossy_impl!(self, $ty)
            }
        }
        )*
    };
}

macro_rules! to_x_impl_suites {
    ($([$ty: ty, $ty_literal: literal]), +$(,)?) => {
        cfg_alloc!(to_x_vec_impl_suite!($([$ty, $ty_literal],)*););
        cfg_alloc!(to_x_slice_lossy_impl_suite!($([$ty, $ty_literal],)*););
        to_x_slice_impl_suite!(from_raw_parts, AsBytesRef::as_bytes_ref::as_ptr, const, $([$ty, $ty_literal],)*);
    };
}

// const MAX_BRUTE_FORCE: usize = 64;

/// Converts to `&'a [u8]`
pub trait AsBytesRef {
    /// Converts to a u8 slice
    fn as_bytes_ref(&self) -> &[u8];
}

/// Converts to `&'a mut [u8]`
pub trait AsBytesMutRef: AsBytesRef {
    /// Converts to a u8 slice
    fn as_bytes_mut_ref(&mut self) -> &mut [u8];
}

/// Extensions for bytes
pub trait BytesExt: AsBytesRef {
    /// Returns whether the underlying bytes is equal
    #[inline]
    fn bytes_eq(&self, other: impl AsBytesRef) -> bool {
        self.as_bytes_ref().eq(other.as_bytes_ref())
    }

    // /// Returns all of the index of the instance of sep in self, or None if sep is not present in s.
    // fn grep_sub_indexes(&self, sep: impl AsBytesRef) -> Option<Vec<usize>> {
    //     let b = self.as_bytes_ref();
    //     let bl = b.len();
    //     let sep = sep.as_bytes_ref();
    //     let n = sep.len();
    //
    //     // when len if small, brute force is ok
    //     if bl <= MAX_BRUTE_FORCE {
    //         let mut vk = Vec::new();
    //         for i in 0..(bl - n + 1) {
    //             let mut ctr = 0;
    //             for j in 0..(n + 1) {
    //                 if b[i + j] != sep[j] {
    //                     ctr = j;
    //                     break;
    //                 }
    //             }
    //             if ctr == n {
    //                 vk.push(i);
    //             }
    //         }
    //         return Some(vk);
    //     }
    //
    //     // TODO: implement Boyer-Moore algorithm when we need to search in large byte slice
    //     None
    // }

    impl_psfix_suites!(AsBytesRef::as_bytes_ref, u8, "u8");

    to_x_impl_suites!(
        [u16, "u16"],
        [u32, "u32"],
        [usize, "usize"],
        [u64, "u64"],
        [u128, "u128"],
        [i8, "i8"],
        [i16, "i16"],
        [i32, "i32"],
        [i64, "i64"],
        [isize, "isize"],
        [i128, "i128"],
        [f32, "f32"],
        [f64, "f64"]
    );
}

/// Extensions for mutable bytes
pub trait BytesMutExt: AsBytesMutRef + BytesExt {
    to_x_slice_impl_suite!(
        mut from_raw_parts_mut,
        AsBytesMutRef::as_bytes_mut_ref::as_mut_ptr,
        mut,
        [u16, "u16"],
        [u32, "u32"],
        [usize, "usize"],
        [u64, "u64"],
        [u128, "u128"],
        [i8, "i8"],
        [i16, "i16"],
        [i32, "i32"],
        [i64, "i64"],
        [isize, "isize"],
        [i128, "i128"],
        [f32, "f32"],
        [f64, "f64"]
    );
}

impl<'a> AsBytesRef for &'a [u8] {
    fn as_bytes_ref(&self) -> &[u8] {
        self
    }
}

impl<'a> BytesExt for &'a [u8] {}

impl<'a> AsBytesRef for &'a mut [u8] {
    fn as_bytes_ref(&self) -> &[u8] {
        self
    }
}

impl<'a> AsBytesMutRef for &'a mut [u8] {
    fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
        self
    }
}

impl<'a> BytesExt for &'a mut [u8] {}

impl<'a> BytesMutExt for &'a mut [u8] {}

impl<const N: usize> AsBytesRef for [u8; N] {
    fn as_bytes_ref(&self) -> &[u8] {
        self
    }
}

impl<const N: usize> AsBytesMutRef for [u8; N] {
    fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
        self
    }
}

impl<const N: usize> BytesExt for [u8; N] {}

impl<const N: usize> BytesMutExt for [u8; N] {}

cfg_alloc! {
    impl AsBytesRef for Box<[u8]> {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl AsBytesMutRef for Box<[u8]> {
        fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
            self.as_mut()
        }
    }

    impl BytesExt for Box<[u8]> {}

    impl BytesMutExt for Box<[u8]> {}

    impl<'a> AsBytesRef for &'a Box<[u8]> {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl<'a> BytesExt for &'a Box<[u8]> {}

    impl<'a> AsBytesRef for &'a mut Box<[u8]> {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl<'a> AsBytesMutRef for &'a mut Box<[u8]> {
        fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
            self.as_mut()
        }
    }

    impl<'a> BytesExt for &'a mut Box<[u8]> {}

    impl<'a> BytesMutExt for &'a mut Box<[u8]> {}


    impl<'a> AsBytesRef for &'a Vec<u8> {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_slice()
        }
    }

    impl<'a> BytesExt for &'a Vec<u8> {}

    impl<'a> AsBytesRef for &'a mut Vec<u8> {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_slice()
        }
    }

    impl<'a> AsBytesMutRef for &'a mut Vec<u8> {
        fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
            self.as_mut_slice()
        }
    }

    impl<'a> BytesExt for &'a mut Vec<u8> {}

    impl<'a> BytesMutExt for &'a mut Vec<u8> {}

    impl AsBytesRef for Vec<u8> {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_slice()
        }
    }

    impl AsBytesMutRef for Vec<u8> {
        fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
            self.as_mut_slice()
        }
    }

    impl BytesExt for Vec<u8> {}

    impl BytesMutExt for Vec<u8> {}
}

impl AsBytesRef for String {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl BytesExt for String {}

impl<'a> AsBytesRef for &'a String {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> AsBytesRef for &'a mut String {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> AsBytesRef for &'a str {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> AsBytesRef for &'a mut str {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> BytesExt for &'a String {}

impl<'a> BytesExt for &'a mut String {}

impl<'a> BytesExt for &'a str {}

impl<'a> BytesExt for &'a mut str {}

cfg_bytes! {
    use bytes::{Bytes, BytesMut};

    impl AsBytesRef for Bytes {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl BytesExt for Bytes {}

    impl<'a> AsBytesRef for &'a Bytes {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl<'a> BytesExt for &'a Bytes {}

    impl AsBytesRef for BytesMut {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl AsBytesMutRef for BytesMut {
        fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
            self.as_mut()
        }
    }

    impl BytesExt for BytesMut {}

    impl BytesMutExt for BytesMut {}

    impl<'a> AsBytesRef for &'a BytesMut {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl<'a> BytesExt for &'a BytesMut {}

    impl<'a> AsBytesRef for &'a mut BytesMut {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl<'a> AsBytesMutRef for &'a mut BytesMut {
        fn as_bytes_mut_ref(&mut self) -> &mut [u8] {
            self.as_mut()
        }
    }

    impl<'a> BytesExt for &'a mut BytesMut {}

    impl<'a> BytesMutExt for &'a mut BytesMut {}

}

#[cfg(test)]
mod tests {
    use super::BytesExt;

    #[test]
    fn test_has_prefix() {
        let a = "Hello, LazyExt!";
        let b = "Hello";
        assert!(a.has_prefix(b));
    }

    #[test]
    fn test_has_suffix() {
        let a = "Hello, LazyExt!";
        let b = "LazyExt!";
        assert!(a.has_suffix(b));
    }

    #[test]
    fn test_longest_prefix() {
        let a = "Hello, LazyExt!";
        let b = "Hello, Rust!";
        assert_eq!(a.longest_prefix(b).len(), "Hello, ".len());
    }

    #[test]
    fn test_longest_suffix() {
        let a = "Hello, LazyExt!";
        let b = "Hi, LazyExt!";
        assert_eq!(a.longest_suffix(b).len(), ", LazyExt!".len());
        assert_eq!(b.longest_suffix(a).len(), ", LazyExt!".len());

        let a = "Hello, LazyExt!";
        let b = "LazyExt!";
        assert_eq!(a.longest_suffix(b).len(), "LazyExt!".len());
        assert_eq!(b.longest_suffix(a).len(), "LazyExt!".len());
    }

    #[test]
    fn test_to_u16() {
        let a = vec![0u8, 1, 0, 2];
        assert_eq!(a.to_be_u16_vec(), vec![1u16, 2u16]);

        let a = vec![1u8, 0, 2, 0];
        assert_eq!(a.to_le_u16_vec(), vec![1u16, 2u16]);
        assert_eq!(a.to_ne_u16_vec().as_slice(), a.to_u16_slice());
        eprintln!(
            "{:?} {:?} {:?}",
            a.to_be_u32_vec(),
            a.to_ne_u32_vec(),
            a.to_le_u32_vec()
        )
    }
}
