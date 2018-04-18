macro_rules! m_convert {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T),*> convert::From<($($T,)*)> for $Tuple<$($T),*> {
            #[inline(always)]
            fn from(t: ($($T,)*)) -> Self {
                $Tuple( $( t.$idx ),* )
            }
        }
        impl<$($T),*> convert::Into<($($T,)*)> for $Tuple<$($T),*> {
            #[inline(always)]
            fn into(self) -> ($($T,)*) {
                ( $( self.$idx, )* )
            }
        }
        
        /// This is only avaible with the 'nightly' feature
        impl<T> convert::From<[T; $(a!(1, $idx)+)* 0]> for $Tuple<$(A!(T, $T)),*> {
            #[inline(always)]
            fn from(t: [T; $(a!(1, $idx)+)* 0]) -> Self {
                #[cfg(feature="nightly")]
                {
                    let [$($T),*] = { t };
                    $Tuple( $( $T, )* )
                }
                #[cfg(not(feature="nightly"))]
                unsafe {
                    use core::ptr;
                    use core::mem;
                    let tuple = $Tuple( $( ptr::read(&t[$idx]), )* );
                    mem::forget(t);
                    tuple
                }
            }
        }
        
        impl<T> convert::Into<[T; 0 $(+ a!(1, $idx))*]> for $Tuple<$(A!(T, $T)),*> {
            #[inline(always)]
            fn into(self) -> [T; 0 $(+ a!(1, $idx))*] {
                let $Tuple($($T),*) = self;
                [ $($T),* ]
            }
        }
        impl<'a, T> $Tuple<$(A!(T, $T)),*> where T: Clone {
            #[inline(always)]
            pub fn from_slice(slice: &'a [T]) -> Option<Self> {
                const N: usize = $(a!(1, $idx)+)* 0;                
                if slice.len() >= N {
                    Some($Tuple( $( slice[$idx].clone() ),* ))
                } else {
                    None
                }
            }
        }
    )*)
}

use core::convert;
use super::*;
impl_tuple!(m_convert);
