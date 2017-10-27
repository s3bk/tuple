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
        impl<T> convert::From<[T; $(a!(1, $idx)+)* 0]> for $Tuple<$(A!(T, $T)),*> {
            #[inline(always)]
            fn from(t: [T; $(a!(1, $idx)+)* 0]) -> Self {
                let [$($T),*] = { t };
                $Tuple( $( $T, )* )
            }
        }
        impl<T> convert::Into<[T; 0 $(+ a!(1, $idx))*]> for $Tuple<$(A!(T, $T)),*> {
            #[inline(always)]
            fn into(self) -> [T; 0 $(+ a!(1, $idx))*] {
                let $Tuple($($T),*) = self;
                [ $($T),* ]
            }
        }
    )*)
}

use core::convert;
use super::*;
impl_tuple!(m_convert);
