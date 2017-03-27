macro_rules! m_tuple {
    ($($Tuple:ident { $($idx:tt -> $T:ident),* } )*) => ($(
        impl<$($T),*> From<($($T,)*)> for $Tuple<$($T),*> {
            fn from(t: ($($T,)*)) -> Self {
                $Tuple( $( t.$idx ),* )
            }
        }
        impl<$($T),*> Into<($($T,)*)> for $Tuple<$($T),*> {
            fn into(self) -> ($($T,)*) {
                ( $( self.$idx, )* )
            }
        }
        impl<T> TupleElements for ($(A!(T,$T),)*) {
            type Element = T;
            const N: usize = $(a!(1, $idx)+)* 0;
            fn elements(&self) -> Elements<&Self> {
                Elements { tuple: self, index: 0 }
            }
            fn get(&self, index: usize) -> Option<&T> {
                match index {
                 $( $idx => Some(&self.$idx), )*
                    _ => None
                }
            }
            fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                match index {
                 $( $idx => Some(&mut self.$idx), )*
                    _ => None
                }
            }
        }
    )*)
}
