macro_rules! repeat_ident {
    (  1 $mac:ident; $($args:ident),*; $($b:ident),* ) => (                  $mac!($($args),* ; $($b),*)          );
    (  2 $mac:ident; $($args:ident),*; $($b:ident),* ) => ( repeat_ident!( 1 $mac; $($args),* ; $($b),* $(,$b)* ) );
    (  4 $mac:ident; $($args:ident),*; $($b:ident),* ) => ( repeat_ident!( 2 $mac; $($args),* ; $($b),* $(,$b)* ) );
    (  8 $mac:ident; $($args:ident),*; $($b:ident),* ) => ( repeat_ident!( 4 $mac; $($args),* ; $($b),* $(,$b)* ) );
    ( 16 $mac:ident; $($args:ident),*; $($b:ident),* ) => ( repeat_ident!( 8 $mac; $($args),* ; $($b),* $(,$b)* ) );
    ( 32 $mac:ident; $($args:ident),*; $($b:ident),* ) => ( repeat_ident!(16 $mac; $($args),* ; $($b),* $(,$b)* ) );
}
macro_rules! impl_one {
    ( $Tuple:ident : $name:ident : $n:tt $e:ident ) =>
    (
        impl Splat<$e> for $name {
            #[inline(always)]
            fn splat(e: $e) -> Self {
                $name::splat(e)
            }
        }
        impl From<$name> for repeat_ident!($n x_ty_ident; $Tuple; $e) {
            #[inline(always)]
            fn from(x: $name) -> Self {
                let mut arr = [$e::default(); $n];
                x.store(&mut arr, 0);
                arr.into()
            }
        }
    )
}
macro_rules! impl_simd_types {
    ( $($Tuple:ident : $simd:ident : $n:tt $e:ident),* ) => ( $( impl_one!( $Tuple : $simd : $n $e ); )* )
}

use super::*;
use simd::*;

#[cfg(target_feature = "sse2")]
use simd::x86::sse2::*;

#[cfg(target_feature = "avx")]
use simd::x86::avx::*;

impl_simd_types! {
    T16: i8x16: 16 i8,
    T8:  i16x8:  8 i16,
    T4:  i32x4:  4 i32,
    T4:  f32x4:  4 f32
}


#[cfg(target_feature = "sse2")]
impl_simd_types! {
    T2:  i64x2:  2 i64,
    T2:  f64x2:  2 f64
}
#[cfg(target_feature = "avx")]
impl_simd_types! {
//  T32: i8x32:  32 i32,
    T16: i16x16: 16 i16,
    T8:  i32x8:   8 i32,
    T4:  i64x4:   4 i64,
    T8:  f32x8:   8 f32,
    T4:  f64x4:   4 f64
}
