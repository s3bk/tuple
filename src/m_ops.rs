macro_rules! m_ops_base {
    ( $Tuple:ident { $($T:ident .$t:ident . $idx:tt),* } : $op:ident . $fn:ident ) =>
    (
        impl<$($T),*> $op for $Tuple<$($T),*> where $( $T: $op ),* {
            type Output = $Tuple<$($T::Output),*>;
            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx.$fn( rhs.$idx ) ),* )
            }
        }
        impl<T> $op<T> for $Tuple<$(A!(T, $T)),*> where T: $op + Clone {
            type Output = $Tuple<$(<A!(T, $T) as $op>::Output),*>;
            #[inline(always)]
            fn $fn(self, rhs: T) -> Self::Output {
                $Tuple( $(self.$idx.$fn( rhs.clone() ) ),* )
            }
        }
    )
}
macro_rules! m_ops_base_assign {
    ( $Tuple:ident { $($T:ident .$t:ident . $idx:tt),* } : $op:ident . $fn:ident ) =>
    (
        impl<$($T),*> $op for $Tuple<$($T),*> where $( $T: $op ),* {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
             $( self.$idx.$fn(rhs.$idx); )*
            }
        }
        impl<T> $op<T> for $Tuple<$(A!(T, $T)),*> where T: $op + Clone {
            #[inline(always)]
            fn $fn(&mut self, rhs: T) {
             $( self.$idx.$fn(rhs.clone()); )*
            }
        }
    )
}
macro_rules! m_ops_all {
    ( $Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } :
        $op:ident.$fn:ident, $op_a:ident.$fn_a:ident) =>
    ( 
        m_ops_base!( $Tuple { $($T . $t . $idx),* } : $op.$fn );
        m_ops_base_assign!( $Tuple { $($T . $t . $idx),* } : $op_a.$fn_a );
    )
}
macro_rules! m_ops {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        m_ops_all!( $Tuple { $($T . $t . $idx),* } : Add.add, AddAssign.add_assign );
        m_ops_all!( $Tuple { $($T . $t . $idx),* } : Sub.sub, SubAssign.sub_assign );
        m_ops_all!( $Tuple { $($T . $t . $idx),* } : Mul.mul, MulAssign.mul_assign );
        m_ops_all!( $Tuple { $($T . $t . $idx),* } : Div.div, DivAssign.div_assign );
        
        impl<$($T),*> Neg for $Tuple<$($T),*> where $( $T: Neg ),* {
            type Output = $Tuple<$($T::Output),*>;
            #[inline(always)]
            fn neg(self) -> Self::Output {
                $Tuple( $(self.$idx.neg()),* )
            }
        }
        impl<T> Index<usize> for $Tuple<$(A!(T,$T)),*> {
            type Output = T;
            #[inline(always)]
            fn index(&self, index: usize) -> &T {
                match index {
                 $( $idx => &self.$idx, )*
                    _ => panic!("index {} out of bounds. len is {}.", index, Self::N)
                }
            }
        }
        impl<T> IndexMut<usize> for $Tuple<$(A!(T,$T)),*> {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut T {
                match index {
                 $( $idx => &mut self.$idx, )*
                    _ => panic!("index {} out of bounds. len is {}.", index, Self::N)
                }
            }
        }
    )*)
}

use core::ops::*;
use super::*;
impl_tuple!(m_ops);
