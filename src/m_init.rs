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
        impl<$($T),*> Eq for $Tuple<$($T),*> where $( $T: Eq ),* {}
        impl<$($T),*> fmt::Debug for $Tuple<$($T),*> where $( $T: fmt::Debug ),* {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                ( $(&self.$idx ),* ).fmt(f)
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
    )*)
}
