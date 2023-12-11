extern crate std;

macro_rules! m_std {
    ($($Tuple:ident $Arr:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T,)*> Hash for $Tuple<$($T),*> where $( $T: Hash),* {
            #[inline(always)]
            fn hash<Ha>(&self, state: &mut Ha) where Ha: Hasher {
                $(
                    self.$idx.hash(state);
                )*
            }
        }
    )*)
}

use std::hash::{Hash, Hasher};
use super::*;
impl_tuple!(m_std);
