 use super::*;

macro_rules! m_array {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        unsafe impl<T> TupleElements for [T; $(a!(1, $idx)+)* 0] {
            type Element = T;
            const N: usize = $(a!(1, $idx)+)* 0;
            fn elements(&self) -> Elements<&Self> {
                Elements::new(self)
            }
            fn elements_mut(&mut self) -> Elements<&mut Self> {
                Elements::new(self)
            }
            fn into_elements(self) -> IntoElements<Self> {
                IntoElements::new(self)
            }
            fn get(&self, index: usize) -> Option<&T> {
                if index < Self::N {
                    Some(&self[index])
                } else {
                    None
                }
            }
            fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                if index < Self::N {
                    Some(&mut self[index])
                } else {
                    None
                }
            }
            fn from_iter<I>(mut iter: I) -> Option<Self> where I: Iterator<Item=Self::Element> {
             $( let $T = match iter.next() {
                    Some(v) => v,
                    None => return None
                }; )*
                Some([$($T,)*])
            }
        }
    )* )
}
impl_tuple!(m_array);
