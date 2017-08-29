use super::*;

macro_rules! m_tuple {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T),*> $Tuple<$(Option<$T>),*> {
            pub fn collect(self) -> Option< $Tuple<$($T),*> > {
                match self {
                    $Tuple( $( Some($T) ),* ) => Some( $Tuple( $( $T ),* ) ),
                    _ => None
                }
            }
        }
        unsafe impl<T> TupleElements for $Tuple<$(A!(T,$T),)*> {
            type Element = T;
            const N: usize = $(a!(1, $idx)+)* 0;
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
            fn from_iter<I>(mut iter: I) -> Option<Self> where I: Iterator<Item=Self::Element> {
             $( let $T = match iter.next() {
                    Some(v) => v,
                    None => return None
                }; )*
                Some($Tuple($($T),*))
            }
        }
        impl<T> Splat<T> for $Tuple<$(A!(T,$T),)*> where T: Clone {
            fn splat(t: T) -> Self {
                $Tuple( $( a!(t.clone(), $T) ),* )
            }
        }
        impl<T, U> Map<U> for $Tuple<$(A!(T,$T)),*> {
            type Output = $Tuple<$(A!(U,$T)),*>;
            fn map<F>(self, f: F) -> Self::Output where F: Fn(T) -> U {
                $Tuple($(f(self.$idx)),*)
            }
        }

        unsafe impl<T> TupleElements for ($(A!(T,$T),)*) {
            type Element = T;
            const N: usize = $(a!(1, $idx)+)* 0;
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
            fn from_iter<I>(mut iter: I) -> Option<Self> where I: Iterator<Item=Self::Element> {
             $( let $T = match iter.next() {
                    Some(v) => v,
                    None => return None
                }; )*
                Some(($($T,)*))
            }
        }
        impl<T> Splat<T> for ($(A!(T,$T),)*) where T: Clone {
            fn splat(t: T) -> Self {
                ( $( a!(t.clone(), $T), )* )
            }
        }
        impl<T, U> Map<U> for ($(A!(T,$T),)*) {
            type Output = ($(A!(U,$T),)*);
            fn map<F>(self, f: F) -> Self::Output where F: Fn(T) -> U {
                ($(f(self.$idx),)*)
            }
        }

        impl<$($T),*> OpRotateLeft for $Tuple<$($T),*> {
            type Output = Rot_l!(x_ty_ident, $Tuple; $($T,)*);
            fn rot_l(self) -> Self::Output {
                rot_l!(x_ty_expr, $Tuple; $(self.$idx,)*)
            }
        }
        impl<$($T),*> OpRotateLeft for ($($T,)*) {
            type Output = Rot_l!(x_tuple_ident; $($T,)*);
            fn rot_l(self) -> Self::Output {
                rot_l!(x_tuple_expr; $(self.$idx,)*)
            }
        }
        impl<$($T),*> OpRotateRight for $Tuple<$($T),*> {
            type Output = Rot_r!(x_ty_ident, $Tuple; $($T,)*);
            fn rot_r(self) -> Self::Output {
                rot_r!(x_ty_expr, $Tuple; $(self.$idx,)*)
            }
        }
        impl<$($T),*> OpRotateRight for ($($T,)*) {
            type Output = Rot_r!(x_tuple_ident; $($T,)*);
            fn rot_r(self) -> Self::Output {
                rot_r!(x_tuple_expr; $(self.$idx,)*)
            }
        }
        impl<$($T),*> OpReverse for $Tuple<$($T),*> {
            type Output = Rev!(x_ty_ident, $Tuple; $($T,)*);
            fn reverse(self) -> Self::Output {
                rev!(x_ty_expr, $Tuple; $(self.$idx,)*)
            }
        }
        impl<$($T),*> OpReverse for ($($T,)*) {
            type Output = Rev!(x_tuple_ident; $($T,)*);
            fn reverse(self) -> Self::Output {
                rev!(x_tuple_expr; $(self.$idx,)*)
            }
        }
    )*)
}

macro_rules! impl_join {
    ( $( $L:ident ( $( $l:ident ),* ) | $R:ident ( $( $r:ident ),* )
      => $O:ident ( $( $o:ident ),* ) )*
    ) => ( $(
        impl<$($l,)* $($r),*> OpJoin<$R<$($r),*>> for $L<$($l),*> {
            type Output = $O<$($l,)* $($r),*>;
            fn join(self, rhs: $R<$($r),*>) -> Self::Output {
                let $L($($l),*) = self;
                let $R($($r),*) = rhs;
                $O($($l,)* $($r),*)
            }
        }
        impl<$($l,)* $($r),*> OpJoin<($($r,)*)> for ($($l,)*) {
            type Output = ($($l,)* $($r),*);
            fn join(self, rhs: ($($r,)*)) -> Self::Output {
                let ($($l,)*) = self;
                let ($($r,)*) = rhs;
                ($($l,)* $($r,)*)
            }
        }
        impl<$($l,)* $($r),*> OpSplit<$L<$($l),*>> for $O<$($l,)* $($r),*> {
            type R = $R<$($r),*>;
            fn split(self) -> ($L<$($l),*>, Self::R) {
                let $O($($l,)* $($r),*) = self;
                ( $L($($l),*), $R($($r),*) )
            }
        }
        impl<$($l,)* $($r),*> OpSplit<($($l),*)> for ($($l,)* $($r),*) {
            type R = ($($r),*);
            fn split(self) -> (($($l),*), Self::R) {
                let ($($l,)* $($r),*) = self;
                ( ($($l),*), ($($r),*) )
            }
        }
    )* )
}

/* python3:
a = string.ascii_lowercase
def impl(l, r): return "T{l}({nl}) | T{r}({nr}) => T{o}({no})".format(l=l, r=r, nl=", ".join(a[:l]), nr=", ".join(a[l:l+r]), o=l+r, no=", ".join(a[:l+r]))

for i in range(1,9):
    for j in range(1,9-i):
        print(impl(i, j))

*/
macro_rules! m_join(
    () => ( impl_join!(
        T1(a) | T1(b) => T2(a, b)
        T1(a) | T2(b, c) => T3(a, b, c)
        T1(a) | T3(b, c, d) => T4(a, b, c, d)
        T1(a) | T4(b, c, d, e) => T5(a, b, c, d, e)
        T1(a) | T5(b, c, d, e, f) => T6(a, b, c, d, e, f)
        T1(a) | T6(b, c, d, e, f, g) => T7(a, b, c, d, e, f, g)
        T1(a) | T7(b, c, d, e, f, g, h) => T8(a, b, c, d, e, f, g, h)
        T2(a, b) | T1(c) => T3(a, b, c)
        T2(a, b) | T2(c, d) => T4(a, b, c, d)
        T2(a, b) | T3(c, d, e) => T5(a, b, c, d, e)
        T2(a, b) | T4(c, d, e, f) => T6(a, b, c, d, e, f)
        T2(a, b) | T5(c, d, e, f, g) => T7(a, b, c, d, e, f, g)
        T2(a, b) | T6(c, d, e, f, g, h) => T8(a, b, c, d, e, f, g, h)
        T3(a, b, c) | T1(d) => T4(a, b, c, d)
        T3(a, b, c) | T2(d, e) => T5(a, b, c, d, e)
        T3(a, b, c) | T3(d, e, f) => T6(a, b, c, d, e, f)
        T3(a, b, c) | T4(d, e, f, g) => T7(a, b, c, d, e, f, g)
        T3(a, b, c) | T5(d, e, f, g, h) => T8(a, b, c, d, e, f, g, h)
        T4(a, b, c, d) | T1(e) => T5(a, b, c, d, e)
        T4(a, b, c, d) | T2(e, f) => T6(a, b, c, d, e, f)
        T4(a, b, c, d) | T3(e, f, g) => T7(a, b, c, d, e, f, g)
        T4(a, b, c, d) | T4(e, f, g, h) => T8(a, b, c, d, e, f, g, h)
        T5(a, b, c, d, e) | T1(f) => T6(a, b, c, d, e, f)
        T5(a, b, c, d, e) | T2(f, g) => T7(a, b, c, d, e, f, g)
        T5(a, b, c, d, e) | T3(f, g, h) => T8(a, b, c, d, e, f, g, h)
        T6(a, b, c, d, e, f) | T1(g) => T7(a, b, c, d, e, f, g)
        T6(a, b, c, d, e, f) | T2(g, h) => T8(a, b, c, d, e, f, g, h)
        T7(a, b, c, d, e, f, g) | T1(h) => T8(a, b, c, d, e, f, g, h)
    );
    )
);

impl_tuple!(m_tuple);
m_join!();
