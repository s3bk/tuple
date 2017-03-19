macro_rules! impl_num {
    ($($Tuple:ident { $($idx:tt -> $T:ident),* } )*) => ($(
        impl<$($T),*> num::Zero for $Tuple<$($T),*> where $( $T: num::Zero ),* {
            fn zero() -> Self {
                $Tuple( $( $T::zero() ),* )
            }
            fn is_zero(&self) -> bool {
                $Tuple( $( self.$idx.is_zero() )&&* )
            }
        }
    )*)
}
