//! Thousands of utility functions for slices and vec
#![doc(html_root_url = "https://docs.rs/lazyext-slice/0.0.2")]
#![deny(
    missing_docs,
    warnings,
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
extern crate paste;

macro_rules! cfg_alloc {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "alloc")]
            #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
            $item
        )*
    }
}

macro_rules! has_prefix {
    ($trait:tt::$fn:tt) => {
        /// Returns whether the slice self begins with prefix.
        #[inline]
        fn has_prefix(&self, prefix: impl $trait) -> bool {
            let src = $trait::$fn(self);
            let prefix = $trait::$fn(&prefix);
            let pl = prefix.len();
            if src.len() < pl {
                return false;
            }

            src[0..pl].eq(prefix)
        }
    };
}

macro_rules! has_suffix {
    ($trait:tt::$fn:tt) => {
        /// Returns whether the slice self ends with suffix.
        #[inline]
        fn has_suffix(&self, suffix: impl $trait) -> bool {
            let src = $trait::$fn(self);
            let suffix = $trait::$fn(&suffix);
            let pl = suffix.len() - 1;
            if src.len() <= pl {
                return false;
            }

            src[pl..].eq(suffix)
        }
    };
}

macro_rules! longest_prefix {
    ($trait:tt::$fn:tt, $ty: ty) => {
        /// Finds the longest shared prefix
        #[inline]
        fn longest_prefix(&self, other: impl $trait) -> &[$ty] {
            let k1 = $trait::$fn(self);
            let k2 = $trait::$fn(&other);
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
    };
}

macro_rules! longest_suffix {
    ($trait:tt::$fn:tt, $ty: ty) => {
        /// Finds the longest shared suffix
        #[inline]
        fn longest_suffix(&self, other: impl $trait) -> &[$ty] {
            let k1 = $trait::$fn(self);
            let k1_len = k1.len();
            let k2 = $trait::$fn(&other);
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
            };
        }
    }
}

#[cfg(feature = "alloc")]
macro_rules! longest_prefix_lossy {
    ($trait:tt::$fn:tt, $ty: ty, $ty_literal: literal) => {
        #[doc = concat!("Finds the longest shared prefix, return a Cow<'_, [", $ty_literal, "]>.")]
        #[inline]
        fn longest_prefix_lossy(&self, other: impl $trait) -> Cow<'_, [$ty]> {
            Cow::Borrowed(self.longest_prefix(other))
        }
    };
}

#[cfg(feature = "alloc")]
macro_rules! longest_suffix_lossy {
    ($trait:tt::$fn:tt, $ty: ty, $ty_literal: literal) => {
        #[doc = concat!("Finds the longest shared suffix, return a Cow<'_, [", $ty_literal, "]>.")]
        #[inline]
        fn longest_suffix_lossy(&self, other: impl $trait) -> Cow<'_, [$ty]> {
            Cow::Borrowed(self.longest_suffix(other))
        }
    };
}

macro_rules! impl_psfix_suites {
    ($trait:tt::$fn:tt, $ty: ty, $ty_literal: literal) => {
        has_prefix!($trait::$fn);

        has_suffix!($trait::$fn);

        longest_prefix!($trait::$fn, $ty);

        longest_suffix!($trait::$fn, $ty);

        cfg_alloc!{
            longest_prefix_lossy!($trait::$fn, $ty, $ty_literal);
            longest_suffix_lossy!($trait::$fn, $ty, $ty_literal);
        }
    };
}

mod bytes_ext;
mod slice_ext;

pub use bytes_ext::*;
pub use slice_ext::*;
