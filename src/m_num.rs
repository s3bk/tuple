macro_rules! m_num {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T),*> num::Zero for $Tuple<$($T),*> where $( $T: num::Zero ),* {
            fn zero() -> Self {
                $Tuple( $( $T::zero() ),* )
            }
            fn is_zero(&self) -> bool {
                $( self.$idx.is_zero() )&&*
            }
        }
    )*)   
}

use super::*;
use num;
impl_tuple!(m_num);
