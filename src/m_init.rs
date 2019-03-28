macro_rules! m_init {
    ($($Tuple:ident $Arr:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T),*> Clone for $Tuple<$($T),*> where $( $T: Clone ),* {
            #[inline(always)]
            fn clone(&self) -> Self {
                $Tuple( $( self.$idx.clone() ),* )
            }
        }
        impl<$($T),*> Copy for $Tuple<$($T),*> where $( $T: Copy ),* {}
        
        impl<$($T),*> PartialEq for $Tuple<$($T),*> where $( $T: PartialEq ),* {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                $( self.$idx == other.$idx)&&*
            }
        }
        impl<$($T),*> Eq for $Tuple<$($T),*> where $( $T: Eq ),* {}
        impl<$($T),*> fmt::Debug for $Tuple<$($T),*> where $( $T: fmt::Debug ),* {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_tuple(stringify!($Tuple))
             $( .field(&self.$idx) )*
                .finish()
            }
        }
        impl<$($T),*> Default for $Tuple<$($T),*> where $( $T: Default ),* {
            #[inline(always)]
            fn default() -> Self {
                $Tuple( $( $T::default() ),* )
            }
        }
        impl<$($T),*> From<u16> for $Tuple<$($T),*> where $( $T: From<u16> ),* {
            #[inline(always)]
            fn from(value: u16) -> Self {
                $Tuple( $( $T::from(value) ),* )
            }
        }
    )*)
}

use core::fmt;
use super::*;
impl_tuple!(m_init);
