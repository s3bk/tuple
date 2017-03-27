macro_rules! m_init {
    ($($Tuple:ident { $($idx:tt -> $T:ident),* } )*) => ($(
        pub struct $Tuple<$($T),*>($(pub $T),*);
        impl<$($T),*> Clone for $Tuple<$($T),*> where $( $T: Clone ),* {
            fn clone(&self) -> Self {
                $Tuple( $( self.$idx.clone() ),* )
            }
        }
        impl<$($T),*> Copy for $Tuple<$($T),*> where $( $T: Copy ),* {}
        
        impl<$($T),*> PartialEq for $Tuple<$($T),*> where $( $T: PartialEq ),* {
            fn eq(&self, other: &Self) -> bool {
                $( self.$idx == other.$idx)&&*
            }
        }
        impl<$($T),*> fmt::Debug for $Tuple<$($T),*> where $( $T: fmt::Debug ),* {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                ( $(&self.$idx ),* ).fmt(f)
            }
        }
        impl<$($T),*> Add for $Tuple<$($T),*> where $( $T: Add ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn add(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx + rhs.$idx ),* )
            }
        }
        impl<$($T),*> Sub for $Tuple<$($T),*> where $( $T: Sub ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn sub(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx - rhs.$idx ),* )
            }
        }
        impl<$($T),*> Mul for $Tuple<$($T),*> where $( $T: Mul ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn mul(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx * rhs.$idx ),* )
            }
        }
        impl<$($T),*> Div for $Tuple<$($T),*> where $( $T: Div ),* {
            type Output = $Tuple<$($T::Output),*>;
            fn div(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx / rhs.$idx ),* )
            }
        }
        impl<$($T),*> From<u16> for $Tuple<$($T),*> where $( $T: From<u16> ),* {
            fn from(value: u16) -> Self {
                $Tuple( $( $T::from(value) ),* )
            }
        }
        impl<$($T),*> Iterator for $Tuple<$($T),*> where $( $T: Iterator  ),* {
            type Item = $Tuple<$($T::Item),*>;
            #[allow(non_snake_case)] 
            fn next(&mut self) -> Option<Self::Item> {
                match ( $(self.$idx.next(), )* ) {
                    ( $( Some($T) ,)* ) => Some($Tuple( $($T),* )),
                    _ => None
                }
            }
        }
        impl<T> TupleElements for $Tuple<$(A!(T,$T)),*> {
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
        impl<T> Index<usize> for $Tuple<$(A!(T,$T)),*> {
            type Output = T;
            fn index(&self, index: usize) -> &T {
                match index {
                 $( $idx => &self.$idx, )*
                    _ => panic!("index {} out of bounds. len is {}.", index, Self::N)
                }
            }
        }
        impl<T> IndexMut<usize> for $Tuple<$(A!(T,$T)),*> {
            fn index_mut(&mut self, index: usize) -> &mut T {
                match index {
                 $( $idx => &mut self.$idx, )*
                    _ => panic!("index {} out of bounds. len is {}.", index, Self::N)
                }
            }
        }
    )*)
}
