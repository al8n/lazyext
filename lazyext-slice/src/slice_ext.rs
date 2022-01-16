use crate::BytesExt;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::mem;
use core::slice::{from_raw_parts, from_raw_parts_mut};

#[cfg(feature = "alloc")]
macro_rules! impl_x_to_u8_vec {
    ($this: ident, $trait:tt::$fn:tt, $typ:tt::$conv:tt) => {{
        $trait::$fn($this)
            .iter()
            .flat_map(|v| $typ::$conv(*v))
            .collect::<Vec<_>>()
    }};
}

macro_rules! impl_x_to_u8_slice {
    ($this: ident, $ty: ty, $trait:tt::$fn:tt::$ptr:tt, $builder: ident, $raw_ptr: tt) => {{
        let src = $trait::$fn($this);
        let len = src.len() * mem::size_of::<$ty>();
        unsafe {
            $builder(src.$ptr() as *$raw_ptr u8, len)
        }
    }};
}

#[cfg(feature = "alloc")]
macro_rules! impl_to_x_vec_suite {
    ($from_literal: literal, $([$ty: ty, $to_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Copy ", $from_literal, " slice to ", $to_literal, " vec in big-endian")]
            #[inline]
            fn [<to_be_ $ty _vec>](&self) -> Vec<$ty> {
                self.to_u8_slice().[<to_be_ $ty _vec>]()
            }

            #[doc = concat!("Copy ", $from_literal, " slice to ", $to_literal, " vec in little-endian")]
            #[inline]
            fn [<to_le_ $ty _vec>](&self) -> Vec<$ty> {
                self.to_u8_slice().[<to_le_ $ty _vec>]()
            }

            #[doc = concat!("Copy ", $from_literal, " slice to ", $to_literal, " vec in native-endian")]
            #[inline]
            fn [<to_ne_ $ty _vec>](&self) -> Vec<$ty> {
                self.to_u8_slice().[<to_ne_ $ty _vec>]()
            }
        }
        )*
    };
}

macro_rules! impl_to_x_slice_suite_in {
    ($this: ident, $builder:ident, $trait:tt::$fn:tt::$ptr:tt, $raw_ptr: ident, $ty: ty) => {{
        let src = $trait::$fn($this);
        let src_ptr = src.$ptr();
        let len = src.len() * mem::size_of::<$ty>();
        unsafe { $builder(src_ptr as *$raw_ptr $ty, len) }
    }};
}

macro_rules! impl_to_x_slice_suite {
    ($from_literal: literal, $builder:ident, $trait:tt::$fn:tt::$ptr:tt, $raw_ptr: ident, $([$ty: ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Convert ", $from_literal, " slice to ", $ty_literal, " slice in native-endian(zero-copy)")]
            fn [<to_ $ty _slice>](&self) -> &[$ty] {
                impl_to_x_slice_suite_in!(self, $builder, $trait::$fn::$ptr, $raw_ptr, $ty)
            }
        }
        )*
    };
    (mut $from_literal: literal, $builder:ident, $trait:tt::$fn:tt::$ptr:tt, $raw_ptr: ident, $([$ty: ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Convert mutable ", $from_literal, " slice to mutable", $ty_literal, " slice in native-endian(zero-copy)")]
            fn [<to_ $ty _slice_mut>](&mut self) -> &[$ty] {
                impl_to_x_slice_suite_in!(self, $builder, $trait::$fn::$ptr, $raw_ptr, $ty)
            }
        }
        )*
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_to_x_slice_lossy_suite {
    ($from_literal: literal, $trait:ident, $([$ty:ty, $ty_literal: literal]), +$(,)?) => {
        $(
        paste! {
            #[doc = concat!("Convert ", $from_literal, " slice to Cow<'_, [", $ty_literal, "]> slice in native-endian(zero-copy)")]
            fn [<to_ $ty _slice_lossy>](&self) -> Cow<'_, [$ty]> {
                Cow::Borrowed($trait::[<to_ $ty _slice>](self))
            }
        }
        )*
    };
}

macro_rules! impl_traits_for_slice_type {
    ($ext_trait: ident, $as_trait: ident, $ty: tt) => {
        paste! {
            impl<'a> $as_trait for &'a [$ty] {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self
                }
            }

            impl<const N: usize> $as_trait for [$ty; N] {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self
                }
            }
        }

        impl<'a> $ext_trait for &'a [$ty] {}

        impl<const N: usize> $ext_trait for [$ty; N] {}
    };
}

macro_rules! impl_traits_for_slice_mut_type {
    ($ext_trait: ident, $mut_ext_trait: ident, $as_trait: ident, $as_mut_trait: ident, $ty: tt) => {
        paste! {
            impl<'a> $as_trait for &'a mut [$ty] {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self
                }
            }

            impl<'a> $as_mut_trait for &'a mut [$ty] {
                fn [<as_ $ty _slice_mut>](&mut self) -> &mut [$ty] {
                    self
                }
            }

            impl<const N: usize> $as_mut_trait for [$ty; N] {
                fn [<as_ $ty _slice_mut>](&mut self) -> &mut [$ty] {
                    self
                }
            }
        }

        impl<'a> $ext_trait for &'a mut [$ty] {}

        impl<'a> $mut_ext_trait for &'a mut [$ty] {}

        impl<const N: usize> $mut_ext_trait for [$ty; N] {}
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_traits_for_vec_type {
    ($ext_trait: ident, $as_trait: ident, $ty: tt) => {
        paste! {
            impl<'a> $as_trait for &'a Vec<$ty> {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self.as_slice()
                }
            }

            impl $as_trait for Vec<$ty> {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self.as_slice()
                }
            }
        }

        impl<'a> $ext_trait for &'a Vec<$ty> {}

        impl $ext_trait for Vec<$ty> {}
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_traits_for_vec_mut_type {
    ($ext_trait: ident, $mut_ext_trait: ident, $as_trait: ident, $as_mut_trait: ident, $ty: tt) => {
        paste! {
            impl<'a> $as_trait for &'a mut Vec<$ty> {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self.as_slice()
                }
            }

            impl<'a> $as_mut_trait for &'a mut Vec<$ty> {
                fn [<as_ $ty _slice_mut>](&mut self) -> &mut [$ty] {
                    self.as_mut_slice()
                }
            }

            impl $as_mut_trait for Vec<$ty> {
                fn [<as_ $ty _slice_mut>](&mut self) -> &mut [$ty] {
                    self.as_mut_slice()
                }
            }
        }

        impl<'a> $ext_trait for &'a mut Vec<$ty> {}

        impl<'a> $mut_ext_trait for &'a mut Vec<$ty> {}

        impl $mut_ext_trait for Vec<$ty> {}
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_traits_for_box_type {
    ($ext_trait: ident, $as_trait: ident, $ty: tt) => {
        paste! {
            impl<'a> $as_trait for &'a Box<[$ty]> {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self.as_ref()
                }
            }

            impl $as_trait for Box<[$ty]> {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self.as_ref()
                }
            }
        }

        impl<'a> $ext_trait for &'a Box<[$ty]> {}
        impl $ext_trait for Box<[$ty]> {}
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_traits_for_box_mut_type {
    ($ext_trait: ident, $mut_ext_trait: ident, $as_trait: ident, $as_mut_trait: ident, $ty: tt) => {
        paste! {
            impl<'a> $as_trait for &'a mut Box<[$ty]> {
                fn [<as_ $ty _slice>](&self) -> &[$ty] {
                    self.as_ref()
                }
            }

            impl<'a> $as_mut_trait for &'a mut Box<[$ty]> {
                fn [<as_ $ty _slice_mut>](&mut self) -> &mut [$ty] {
                    self.as_mut()
                }
            }

            impl $as_mut_trait for Box<[$ty]> {
                fn [<as_ $ty _slice_mut>](&mut self) -> &mut [$ty] {
                    self.as_mut()
                }
            }
        }

        impl<'a> $ext_trait for &'a mut Box<[$ty]> {}

        impl<'a> $mut_ext_trait for &'a mut Box<[$ty]> {}
        impl $mut_ext_trait for Box<[$ty]> {}
    };
}

macro_rules! declare_as_x_slice_trait {
    ($([$ext_trait_name: ident, $as_trait_name: ident, $fn_name: ident, $typ: tt, $typ_literal: literal, $([$convert_typ: ty, $convert_typ_literal: literal]), +$(,)?]), +$(,)?) => {
        $(
        #[doc = concat!("Converts to `&'a [", $typ_literal, "]`")]
        pub trait $as_trait_name {
            #[doc = concat!("Converts to ", $typ_literal, " slice")]
            fn $fn_name(&self) -> &[$typ];
        }

        #[doc = concat!("Extensions for ", $typ_literal, " slice")]
        pub trait $ext_trait_name: $as_trait_name {

            impl_psfix_suites!($as_trait_name::$fn_name, $typ, $typ_literal);

            #[doc = concat!("Copy ", $typ_literal, " slice to u8 vec in big-endian")]
            #[cfg(feature = "alloc")]
            fn to_be_u8_vec(&self) -> Vec<u8> {
                impl_x_to_u8_vec!(self, $as_trait_name::$fn_name, $typ::to_be_bytes)
            }

            #[doc = concat!("Copy ", $typ_literal, " slice to u8 vec in little-endian")]
            #[cfg(feature = "alloc")]
            fn to_le_u8_vec(&self) -> Vec<u8> {
                impl_x_to_u8_vec!(self, $as_trait_name::$fn_name, $typ::to_le_bytes)
            }

            #[doc = concat!("Copy ", $typ_literal, " slice to u8 vec in native-endian")]
             #[cfg(feature = "alloc")]
            fn to_ne_u8_vec(&self) -> Vec<u8> {
                impl_x_to_u8_vec!(self, $as_trait_name::$fn_name, $typ::to_ne_bytes)
            }

            cfg_alloc!(impl_to_x_vec_suite!($typ_literal, $([$convert_typ, $convert_typ_literal],)*););

            #[doc = concat!("convert u16 slice to u8 slice")]
            fn to_u8_slice(&self) -> &[u8] {
                impl_x_to_u8_slice!(self, $typ, $as_trait_name::$fn_name::as_ptr, from_raw_parts, const)
            }

            cfg_alloc!(impl_to_x_slice_lossy_suite!($typ_literal, $ext_trait_name, [u8, "u8"], $([$convert_typ, $convert_typ_literal],)*););

            impl_to_x_slice_suite!($typ_literal, from_raw_parts, $ext_trait_name::to_u8_slice::as_ptr, const, $([$convert_typ, $convert_typ_literal],)*);
        }

        impl_traits_for_slice_type!($ext_trait_name, $as_trait_name, $typ);
        impl_traits_for_vec_type!($ext_trait_name, $as_trait_name, $typ);
        impl_traits_for_box_type!($ext_trait_name, $as_trait_name, $typ);
        )*
    };
}

macro_rules! declare_as_x_slice_mut_trait {
    ($([$ext_trait_name: ident, $mut_ext_trait_name: ident, $as_trait_name: ident, $as_mut_trait_name: ident, $fn_mut_name: ident, $typ: ty, $typ_literal: literal, $([$convert_typ: ty, $convert_typ_literal: literal]), +$(,)?]), +$(,)?) => {
        $(
        #[doc = concat!("Converts to `&'a mut [", $typ_literal, "]`")]
        pub trait $as_mut_trait_name: $as_trait_name  {
            #[doc = concat!("Converts to mutable ", $typ_literal, " slice")]
            fn $fn_mut_name(&mut self) -> &mut [$typ];
        }

        #[doc = concat!("Extensions for mutable ", $typ_literal, " slice")]
        pub trait $mut_ext_trait_name: $as_mut_trait_name + $ext_trait_name {
            #[doc = concat!("convert ", $typ_literal, " slice to mutable u8 slice")]
            fn to_u8_slice_mut(&mut self) -> &mut [u8] {
                impl_x_to_u8_slice!(self, $typ, $as_mut_trait_name::$fn_mut_name::as_mut_ptr, from_raw_parts_mut, mut)
            }

            impl_to_x_slice_suite!(mut $typ_literal, from_raw_parts_mut, $mut_ext_trait_name::to_u8_slice_mut::as_mut_ptr, mut, [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]);
        }

        impl_traits_for_slice_mut_type!($ext_trait_name, $mut_ext_trait_name, $as_trait_name, $as_mut_trait_name, $typ);
        impl_traits_for_vec_mut_type!($ext_trait_name, $mut_ext_trait_name, $as_trait_name, $as_mut_trait_name, $typ);
        impl_traits_for_box_mut_type!($ext_trait_name, $mut_ext_trait_name, $as_trait_name, $as_mut_trait_name, $typ);
        )*
    };
}

declare_as_x_slice_trait! {
    [U16SliceExt, AsU16Slice, as_u16_slice, u16, "u16", [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [U32SliceExt, AsU32Slice, as_u32_slice, u32, "u32", [u16, "u16"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [USizeSliceExt, AsUSizeSlice, as_usize_slice, usize, "usize", [u16, "u16"], [u32, "u32"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [U64SliceExt, AsU64Slice, as_u64_slice, u64, "u64", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [U128SliceExt, AsU128Slice, as_u128_slice, u128, "u128", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I8SliceExt, AsI8Slice, as_i8_slice, i8, "i8", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I16SliceExt, AsI16Slice, as_i16_slice, i16, "i16", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I32SliceExt, AsI32Slice, as_i32_slice, i32, "i32", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [ISizeSliceExt, AsISizeSlice, as_isize_slice, isize, "isize", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I64SliceExt, AsI64Slice, as_i64_slice, i64, "i64", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I128SliceExt, AsI128Slice, as_i128_slice, i128, "i128", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [f32, "f32"], [f64, "f64"]],

    [F32SliceExt, AsF32Slice, as_f32_slice, f32, "f32", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f64, "f64"]],

    [F64SliceExt, AsF64Slice, as_f64_slice, f64, "f64", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"]],
}

declare_as_x_slice_mut_trait! {
    [U16SliceExt, U16SliceMutExt, AsU16Slice, AsU16SliceMut, as_u16_slice_mut, u16, "u16", [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [U32SliceExt, U32SliceMutExt, AsU32Slice, AsU32SliceMut, as_u32_slice_mut, u32, "u32", [u16, "u16"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [USizeSliceExt, USizeSliceMutExt, AsUSizeSlice, AsUSizeSliceMut, as_usize_slice_mut, usize, "usize", [u16, "u16"], [u32, "u32"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [U64SliceExt, U64SliceMutExt, AsU64Slice, AsU64SliceMut, as_u64_slice_mut, u64, "u64", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [U128SliceExt, U128SliceMutExt, AsU128Slice, AsU128SliceMut, as_u128_slice_mut, u128, "u128", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I8SliceExt, I8SliceMutExt, AsI8Slice, AsI8SliceMut, as_i8_slice_mut, i8, "i8", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I16SliceExt, I16SliceMutExt, AsI16Slice, AsI16SliceMut, as_i16_slice_mut, i16, "i16", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I32SliceExt, I32SliceMutExt, AsI32Slice, AsI32SliceMut, as_i32_slice_mut, i32, "i32", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [ISizeSliceExt, ISizeSliceMutExt, AsISizeSlice, AsISizeSliceMut, as_isize_slice_mut, isize, "isize", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I64SliceExt, I64SliceMutExt, AsI64Slice, AsI64SliceMut, as_i64_slice_mut, i64, "i64", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [isize, "isize"], [i128, "i128"], [f32, "f32"], [f64, "f64"]],

    [I128SliceExt, I128SliceMutExt, AsI128Slice, AsI128SliceMut, as_i128_slice_mut, i128, "i128", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [f32, "f32"], [f64, "f64"]],

    [F32SliceExt, F32SliceMutExt, AsF32Slice, AsF32SliceMut, as_f32_slice_mut, f32, "f32", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"],  [f64, "f64"]],

    [F64SliceExt, F64SliceMutExt, AsF64Slice, AsF64SliceMut, as_f64_slice_mut, f64, "f64", [u16, "u16"], [u32, "u32"], [usize, "usize"], [u64, "u64"], [u128, "u128"], [i8, "i8"], [i16, "i16"], [i32, "i32"], [i64, "i64"], [isize, "isize"], [i128, "i128"], [f32, "f32"]],
}

#[cfg(test)]
mod tests {
    use crate::slice_ext::U16SliceExt;

    #[test]
    fn test_slice() {
        let u16s = [1u16; 12];
        let u8v = u16s.as_slice();
        eprintln!(
            "{:?} {:?} {:?}",
            u8v.to_be_u32_vec(),
            u8v.to_le_u32_vec(),
            u8v.to_ne_u32_vec()
        );
        eprintln!("{:?}", u8v.to_u32_slice());
    }
}
