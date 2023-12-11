use super::*;

macro_rules! m_call {
    ($($Tuple:ident $Arr:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<Func, Out, $($T),*> Call<$Tuple<$($T),*>> for Func where Func: Fn($($T),*) -> Out {
            type Output = Out;
            #[inline(always)]
            fn call(&self, $Tuple($($t),*): $Tuple<$($T),*>) -> Out {
                self($($t),*)
            }
        }
        impl<Func, Out, $($T),*> CallMut<$Tuple<$($T),*>> for Func where Func: FnMut($($T),*) -> Out {
            type Output = Out;
            #[inline(always)]
            fn call_mut(&mut self, $Tuple($($t),*): $Tuple<$($T),*>) -> Out {
                self($($t),*)
            }
        }
        impl<Func, Out, $($T),*> CallOnce<$Tuple<$($T),*>> for Func where Func: FnOnce($($T),*) -> Out {
            type Output = Out;
            #[inline(always)]
            fn call_once(self, $Tuple($($t),*): $Tuple<$($T),*>) -> Out {
                self($($t),*)
            }
        }
        impl<Func, Out, $($T),*> Call<($($T,)*)> for Func where Func: Fn($($T),*) -> Out {
            type Output = Out;
            #[inline(always)]
            fn call(&self, ($($t,)*): ($($T,)*)) -> Out {
                self($($t),*)
            }
        }
        impl<Func, Out, $($T),*> CallMut<($($T,)*)> for Func where Func: FnMut($($T),*) -> Out {
            type Output = Out;
            #[inline(always)]
            fn call_mut(&mut self, ($($t,)*): ($($T,)*)) -> Out {
                self($($t),*)
            }
        }
        impl<Func, Out, $($T),*> CallOnce<($($T,)*)> for Func where Func: FnOnce($($T),*) -> Out {
            type Output = Out;
            #[inline(always)]
            fn call_once(self, ($($t,)*): ($($T,)*)) -> Out {
                self($($t),*)
            }
        }
    )*)
}

impl_tuple!(m_call);
