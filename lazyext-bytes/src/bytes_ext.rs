use core::mem;
use core::ptr::slice_from_raw_parts;

macro_rules! cfg_bytes {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "bytes")]
            #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
            $item
        )*
    }
}

macro_rules! to_x_vec_impl {
    ($this:ident, $typ:tt::$conv:tt) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        let src = $this.as_bytes_ref();
        assert_eq!(src.len() % SIZE, 0, "invalid length of u8 slice: {}", src.len());
        let ptr = src.as_ptr();
        (0..src.len()).step_by(SIZE).map(|v| unsafe { $typ::$conv(*(ptr.add(v) as *const _ as *const [_; SIZE]))}).collect::<Vec<_>>()
    }};
}

macro_rules! to_x_slice_impl {
    ($this:ident, $typ: ident) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        let src = $this.as_bytes_ref();
        assert_eq!(src.len() % SIZE, 0, "invalid length of u8 slice: {}", src.len());
        let ptr = src.as_ptr() as *const $typ;
        unsafe {
            &*slice_from_raw_parts(ptr, src.len() / SIZE)
        }
    }};
}

const MAX_BRUTE_FORCE: usize = 64;

/// convert to `&'a [u8]`
pub trait AsBytesRef {
    /// converts to a u8 slice
    fn as_bytes_ref(&self) -> &[u8];
}

impl<'a> AsBytesRef for &'a [u8] {
    fn as_bytes_ref(&self) -> &[u8] {
        self
    }
}

impl<'a> AsBytesRef for &'a mut [u8] {
    fn as_bytes_ref(&self) -> &[u8] {
        self
    }
}

impl AsBytesRef for Box<[u8]> {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<'a> AsBytesRef for &'a Box<[u8]> {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<'a> AsBytesRef for &'a mut Box<[u8]> {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<const N: usize> AsBytesRef for [u8; N] {
    fn as_bytes_ref(&self) -> &[u8] {
        self
    }
}

impl<'a> AsBytesRef for &'a Vec<u8> {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsBytesRef for &'a mut Vec<u8> {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsBytesRef for Vec<u8> {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsBytesRef for String {
    fn as_bytes_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

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

    /// Returns whether the byte slice s begins with prefix.
    #[inline]
    fn has_prefix(&self, prefix: impl AsBytesRef) -> bool {
        let src = self.as_bytes_ref();
        let prefix = prefix.as_bytes_ref();
        let pl = prefix.len();
        if src.len() < pl {
            return false;
        }

        src[0..pl].eq(prefix)
    }

    /// Returns whether the byte slice s ends with suffix.
    #[inline]
    fn has_suffix(&self, suffix: impl AsBytesRef) -> bool {
        let src = self.as_bytes_ref();
        let suffix = suffix.as_bytes_ref();
        let pl = suffix.len() - 1;
        if src.len() <= pl {
            return false;
        }

        src[pl..].eq(suffix)
    }

    /// Finds the longest shared prefix
    #[inline]
    fn longest_prefix(&self, other: impl AsBytesRef) -> &[u8] {
        let k1 = self.as_bytes_ref();
        let k2 = other.as_bytes_ref();
        let max = k1.len().min(k2.len());

        let mut n = max - 1;
        for i in 0..max {
            if k1[i].ne(&k2[i]) {
                n = i;
                break;
            }
        }
        &k1[..n]
    }

    /// Finds the longest shared suffix
    #[inline]
    fn longest_suffix(&self, other: impl AsBytesRef) -> &[u8] {
        let k1 = self.as_bytes_ref();
        let k1_len = k1.len();
        let k2 = other.as_bytes_ref();
        let k2_len = k2.len();
        return if k1_len < k2_len {
            let max = k1_len;
            let mut n = max;
            for i in 0..max {
                if k1[k1_len - i - 1].ne(&k2[k2_len - i - 1]) {
                    n = i;
                    break;
                }
            }
            &k1[max - n..]
        } else {
            let max = k2_len;
            let mut n = max;
            for i in 0..max {
                if k1[k1_len - i - 1].ne(&k2[k2_len - i - 1]) {
                    n = i;
                    break;
                }
            }
            &k1[k1_len - k2_len + max - n..]
        }
    }

    /// convert u8 slice to u16 slice in native-endian (zero-copy)
    #[inline]
    fn to_u16_slice(&self) -> &[u16] {
        to_x_slice_impl!(self, u16)
    }

    /// convert u8 slice to u32 slice in native-endian (zero-copy)
    #[inline]
    fn to_u32_slice(&self) -> &[u32] {
        to_x_slice_impl!(self, u32)
    }

    /// convert u8 slice to usize slice in native-endian (zero-copy)
    #[inline]
    fn to_usize_slice(&self) -> &[usize] {
        to_x_slice_impl!(self, usize)
    }

    /// convert u8 slice to u64 slice in native-endian (zero-copy)
    #[inline]
    fn to_u64_slice(&self) -> &[u64] {
        to_x_slice_impl!(self, u64)
    }

    /// convert u8 slice to u128 slice in native-endian (zero-copy)
    #[inline]
    fn to_u128_slice(&self) -> &[u128] {
        to_x_slice_impl!(self, u128)
    }

    /// convert u8 slice to i8 slice in native-endian (zero-copy)
    #[inline]
    fn to_i8_slice(&self) -> &[i8] {
        to_x_slice_impl!(self, i8)
    }

    /// convert u8 slice to i16 slice in native-endian (zero-copy)
    #[inline]
    fn to_i16_slice(&self) -> &[i16] {
        to_x_slice_impl!(self, i16)
    }

    /// convert u8 slice to i8 slice in native-endian (zero-copy)
    #[inline]
    fn to_i32_slice(&self) -> &[i32] {
        to_x_slice_impl!(self, i32)
    }

    /// convert u8 slice to isize slice in native-endian (zero-copy)
    #[inline]
    fn to_isize_slice(&self) -> &[isize] {
        to_x_slice_impl!(self, isize)
    }

    /// convert u8 slice to i64 slice in native-endian (zero-copy)
    #[inline]
    fn to_i64_slice(&self) -> &[i64] {
        to_x_slice_impl!(self, i64)
    }

    /// convert u8 slice to i128 slice in native-endian (zero-copy)
    #[inline]
    fn to_i128_slice(&self) -> &[i128] {
        to_x_slice_impl!(self, i128)
    }

    /// convert u8 slice to f32 slice in native-endian (zero-copy)
    #[inline]
    fn to_f32_slice(&self) -> &[f32] {
        to_x_slice_impl!(self, f32)
    }

    /// convert u8 slice to f64 slice in native-endian (zero-copy)
    #[inline]
    fn to_f64_slice(&self) -> &[f64] {
        to_x_slice_impl!(self, f64)
    }

    /// Copy u8 slice to u16 vec in big-endian
    #[inline]
    fn to_be_u16_vec(&self) -> Vec<u16> {
        to_x_vec_impl!(self, u16::from_be_bytes)
    }

    /// Copy u8 slice to u16 vec in little-endian
    #[inline]
    fn to_le_u16_vec(&self) -> Vec<u16> {
        to_x_vec_impl!(self, u16::from_le_bytes)
    }

    /// Copy u8 slice to u16 vec in native-endian
    #[inline]
    fn to_ne_u16_vec(&self) -> Vec<u16> {
        to_x_vec_impl!(self, u16::from_ne_bytes)
    }

    /// Copy u8 slice to u32 vec in big-endian
    #[inline]
    fn to_be_u32_vec(&self) -> Vec<u32> {
        to_x_vec_impl!(self, u32::from_be_bytes)
    }

    /// Copy u8 slice to u32 vec in little-endian
    #[inline]
    fn to_le_u32_vec(&self) -> Vec<u32> {
        to_x_vec_impl!(self, u32::from_le_bytes)
    }

    /// Copy u8 slice to u16 vec in native-endian
    #[inline]
    fn to_ne_u32_vec(&self) -> Vec<u32> {
        to_x_vec_impl!(self, u32::from_ne_bytes)
    }

    /// Copy u8 slice to usize vec in big-endian
    #[inline]
    fn to_be_usize_vec(&self) -> Vec<usize> {
        to_x_vec_impl!(self, usize::from_be_bytes)
    }

    /// Copy u8 slice to usize vec in little-endian
    #[inline]
    fn to_le_usize_vec(&self) -> Vec<usize> {
        to_x_vec_impl!(self, usize::from_le_bytes)
    }

    /// Copy u8 slice to usize vec in native-endian
    #[inline]
    fn to_ne_usize_vec(&self) -> Vec<usize> {
        to_x_vec_impl!(self, usize::from_ne_bytes)
    }

    /// Copy u8 slice to u64 vec in big-endian
    #[inline]
    fn to_be_u64_vec(&self) -> Vec<u64> {
        to_x_vec_impl!(self, u64::from_be_bytes)
    }

    /// Copy u8 slice to u64 vec in little-endian
    #[inline]
    fn to_le_u64_vec(&self) -> Vec<u64> {
        to_x_vec_impl!(self, u64::from_le_bytes)
    }

    /// Copy u8 slice to u64 vec in native-endian
    #[inline]
    fn to_ne_u64_vec(&self) -> Vec<u64> {
        to_x_vec_impl!(self, u64::from_ne_bytes)
    }

    /// Copy u8 slice to u128 vec in big-endian
    #[inline]
    fn to_be_u128_vec(&self) -> Vec<u128> {
        to_x_vec_impl!(self, u128::from_be_bytes)
    }

    /// Copy u8 slice to u128 vec in little-endian
    #[inline]
    fn to_le_u128_vec(&self) -> Vec<u128> {
        to_x_vec_impl!(self, u128::from_le_bytes)
    }

    /// Copy u8 slice to u16 vec in native-endian
    #[inline]
    fn to_ne_u128_vec(&self) -> Vec<u128> {
        to_x_vec_impl!(self, u128::from_ne_bytes)
    }

    /// Copy u8 slice to i8 vec in big-endian
    #[inline]
    fn to_be_i8_vec(&self) -> Vec<i8> {
        to_x_vec_impl!(self, i8::from_be_bytes)
    }

    /// Copy u8 slice to i8 vec in little-endian
    #[inline]
    fn to_le_i8_vec(&self) -> Vec<i8> {
        to_x_vec_impl!(self, i8::from_le_bytes)
    }

    /// Copy u8 slice to i8 vec in native-endian
    #[inline]
    fn to_ne_i8_vec(&self) -> Vec<i8> {
        to_x_vec_impl!(self, i8::from_ne_bytes)
    }

    /// Copy u8 slice to i16 vec in big-endian
    #[inline]
    fn to_be_i16_vec(&self) -> Vec<i16> {
        to_x_vec_impl!(self, i16::from_be_bytes)
    }

    /// Copy u8 slice to i16 vec in little-endian
    #[inline]
    fn to_le_i16_vec(&self) -> Vec<i16> {
        to_x_vec_impl!(self, i16::from_le_bytes)
    }

    /// Copy u8 slice to i16 vec in native-endian
    #[inline]
    fn to_ne_i16_vec(&self) -> Vec<i16> {
        to_x_vec_impl!(self, i16::from_ne_bytes)
    }

    /// Copy u8 slice to i32 vec in big-endian
    #[inline]
    fn to_be_i32_vec(&self) -> Vec<i32> {
        to_x_vec_impl!(self, i32::from_be_bytes)
    }

    /// Copy u8 slice to i32 vec in little-endian
    #[inline]
    fn to_le_i32_vec(&self) -> Vec<i32> {
        to_x_vec_impl!(self, i32::from_le_bytes)
    }

    /// Copy u8 slice to i16 vec in native-endian
    #[inline]
    fn to_le_n32_vec(&self) -> Vec<i32> {
        to_x_vec_impl!(self, i32::from_ne_bytes)
    }

    /// Copy u8 slice to isize vec in big-endian
    #[inline]
    fn to_be_isize_vec(&self) -> Vec<isize> {
        to_x_vec_impl!(self, isize::from_be_bytes)
    }

    /// Copy u8 slice to isize vec in little-endian
    #[inline]
    fn to_le_isize_vec(&self) -> Vec<isize> {
        to_x_vec_impl!(self, isize::from_le_bytes)
    }

    /// Copy u8 slice to isize vec in native-endian
    #[inline]
    fn to_ne_isize_vec(&self) -> Vec<isize> {
        to_x_vec_impl!(self, isize::from_ne_bytes)
    }

    /// Copy u8 slice to i64 vec in big-endian
    #[inline]
    fn to_be_i64_vec(&self) -> Vec<i64> {
        to_x_vec_impl!(self, i64::from_be_bytes)
    }

    /// Copy u8 slice to i64 vec in little-endian
    #[inline]
    fn to_le_i64_vec(&self) -> Vec<i64> {
        to_x_vec_impl!(self, i64::from_le_bytes)
    }

    /// Copy u8 slice to i64 vec in native-endian
    #[inline]
    fn to_ne_i64_vec(&self) -> Vec<i64> {
        to_x_vec_impl!(self, i64::from_ne_bytes)
    }

    /// Copy u8 slice to i128 vec in big-endian
    #[inline]
    fn to_be_i128_vec(&self) -> Vec<i128> {
        to_x_vec_impl!(self, i128::from_be_bytes)
    }

    /// Copy u8 slice to i128 vec in little-endian
    #[inline]
    fn to_le_i128_vec(&self) -> Vec<i128> {
        to_x_vec_impl!(self, i128::from_le_bytes)
    }

    /// Copy u8 slice to i128 vec in native-endian
    #[inline]
    fn to_ne_i128_vec(&self) -> Vec<i128> {
        to_x_vec_impl!(self, i128::from_ne_bytes)
    }

    /// Copy u8 slice to f32 vec in big-endian
    #[inline]
    fn to_be_f32_vec(&self) -> Vec<f32> {
        to_x_vec_impl!(self, f32::from_be_bytes)
    }

    /// Copy u8 slice to f32 vec in little-endian
    #[inline]
    fn to_le_f32_vec(&self) -> Vec<f32> {
        to_x_vec_impl!(self, f32::from_le_bytes)
    }

    /// Copy u8 slice to f32 vec in native-endian
    #[inline]
    fn to_ne_f32_vec(&self) -> Vec<f32> {
        to_x_vec_impl!(self, f32::from_ne_bytes)
    }

    /// Copy u8 slice to f64 vec in big-endian
    #[inline]
    fn to_be_f64_vec(&self) -> Vec<f64> {
        to_x_vec_impl!(self, f64::from_be_bytes)
    }

    /// Copy u8 slice to f64 vec in little-endian
    #[inline]
    fn to_le_f64_vec(&self) -> Vec<f64> {
        to_x_vec_impl!(self, f64::from_le_bytes)
    }

    /// Copy u8 slice to f64 vec in native-endian
    #[inline]
    fn to_ne_f64_vec(&self) -> Vec<f64> {
        to_x_vec_impl!(self, f64::from_ne_bytes)
    }
}

impl<'a> BytesExt for &'a [u8] {}

impl<'a> BytesExt for &'a mut [u8] {}

impl<const N: usize> BytesExt for [u8; N] {}

impl BytesExt for Vec<u8> {}

impl<'a> BytesExt for &'a Vec<u8> {}

impl<'a> BytesExt for &'a mut Vec<u8> {}

impl BytesExt for String {}

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

    impl AsBytesRef for BytesMut {
        fn as_bytes_ref(&self) -> &[u8] {
            self.as_ref()
        }
    }

    impl BytesExt for Bytes {}

    impl BytesExt for BytesMut {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}

