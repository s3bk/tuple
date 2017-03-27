macro_rules! m_ops {
    ($($Tuple:ident { $($idx:tt -> $T:ident),* } )*) => ($(
        impl<$($T),*> ops::Add for $Tuple<$($T),*> where $( $T: ops::Add ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn add(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx + rhs.$idx ),* )
            }
        }
        impl<$($T),*> ops::AddAssign for $Tuple<$($T),*> where $( $T: ops::AddAssign ),* {
            fn add_assign(&mut self, rhs: Self) {
             $( self.$idx.add_assign(rhs.$idx); )*
            }
        }
        impl<$($T),*> ops::Sub for $Tuple<$($T),*> where $( $T: ops::Sub ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn sub(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx - rhs.$idx ),* )
            }
        }
        impl<$($T),*> ops::SubAssign for $Tuple<$($T),*> where $( $T: ops::SubAssign ),* {
            fn sub_assign(&mut self, rhs: Self) {
             $( self.$idx.sub_assign(rhs.$idx); )*
            }
        }
        impl<$($T),*> ops::Mul for $Tuple<$($T),*> where $( $T: ops::Mul ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn mul(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx * rhs.$idx ),* )
            }
        }
        impl<$($T),*> ops::Div for $Tuple<$($T),*> where $( $T: ops::Div ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn div(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx / rhs.$idx ),* )
            }
        }
        impl<$($T),*> ops::Neg for $Tuple<$($T),*> where $( $T: ops::Neg ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn neg(self) -> Self::Output {
                $Tuple( $(self.$idx.neg()),* )
            }
        }
        impl<T> ops::Index<usize> for $Tuple<$(A!(T,$T)),*> {
            type Output = T;
            fn index(&self, index: usize) -> &T {
                match index {
                 $( $idx => &self.$idx, )*
                    _ => panic!("index {} out of bounds. len is {}.", index, Self::N)
                }
            }
        }
        impl<T> ops::IndexMut<usize> for $Tuple<$(A!(T,$T)),*> {
            fn index_mut(&mut self, index: usize) -> &mut T {
                match index {
                 $( $idx => &mut self.$idx, )*
                    _ => panic!("index {} out of bounds. len is {}.", index, Self::N)
                }
            }
        }
    )*)
}
