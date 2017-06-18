macro_rules! m_convert {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T),*> convert::From<($($T,)*)> for $Tuple<$($T),*> {
            fn from(t: ($($T,)*)) -> Self {
                $Tuple( $( t.$idx ),* )
            }
        }
        impl<$($T),*> convert::Into<($($T,)*)> for $Tuple<$($T),*> {
            fn into(self) -> ($($T,)*) {
                ( $( self.$idx, )* )
            }
        }
        impl<T> convert::From<[T; $(a!(1, $idx)+)* 0]> for $Tuple<$(A!(T, $T)),*> {
            fn from(t: [T; $(a!(1, $idx)+)* 0]) -> Self {
                let [$($T),*] = { t };
                $Tuple( $( $T, )* )
            }
        }
        impl<T> convert::Into<[T; 0 $(+ a!(1, $idx))*]> for $Tuple<$(A!(T, $T)),*> {
            fn into(self) -> [T; 0 $(+ a!(1, $idx))*] {
                let $Tuple($($T),*) = self;
                [ $($T),* ]
            }
        }
        impl<'a, T> convert::TryFrom<&'a [T]> for $Tuple<$(A!(T, $T)),*> where T: Clone {
            type Error = ConvertError;
            fn try_from(slice: &'a [T]) -> Result<Self, ConvertError> {
                const N: usize = $(a!(1, $idx)+)* 0;
                
                if slice.len() >= N {
                    Ok($Tuple( $( slice[$idx].clone() ),* ))
                } else {
                    Err(ConvertError::OutOfBounds)
                }
            }
        }
        impl<$($T,$t,)*> convert::TryFrom<$Tuple<$($t,)*>> for $Tuple<$($T,)*>
        where $( $T: convert::TryFrom<$t> ),*
        {
            type Error = $Tuple<$(Option<$T::Error>,)*>;
            fn try_from(value: $Tuple<$($t,)*>) -> Result<Self, Self::Error> {
                match ($( $T::try_from(value.$idx), )* ) {
                    ($( Ok($t), )*) => Ok($Tuple( $( $t ),* )),
                    ($( $t, )*) => Err($Tuple( $( $t.err() ),* ))
                }
            }
        }
    )*)
}

use core::convert;
use super::*;
impl_tuple!(m_convert);
