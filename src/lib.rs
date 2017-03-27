/*!
# Examples

```
extern crate tuple;
use tuple::*;
# fn main() {
let a = T2(3, 4) + T2(5, 4);
assert_eq!(a, T2(8, 8));

// Tuples where all items have the same type can be indexed
assert_eq!(T3(1, 2, 3)[2], 3);
assert_eq!(a[1], 8);

let b = T2(3u32, 4.0f32) * T2(7, 3.0);
assert_eq!(b, T2(21, 12.));

# }
```

## Adding a Trait
```
#[macro_use]
extern crate tuple;
extern crate num_traits;

use tuple::*;
use num_traits::Zero;
use std::ops::{Add, Sub, Mul};
use std::fmt::Debug;

trait Ring: Add + Sub + Mul + Zero + Debug + Sized {}

// The name is up to you
macro_rules! impl_ring {
    // This line is defined by this crate and can't be changed
    ($($Tuple:ident { $($idx:tt -> $T:ident),* } )*) => ($(

        // This is expanded for every Tuple type
        impl<$($T),*> Ring for $Tuple<$($T),*> where Self: Zero, $( $T: Ring ),* {}

    // this has to match again
    )*)
}

// actually implement it!
impl_tuple!(impl_ring);

# fn main() {
# }
```
**/

#![feature(associated_consts)]
#![feature(trace_macros)]
#![feature(try_from)] 
#![no_std]

#[cfg(impl_num)]
extern crate num_traits;

//extern crate itertools;


// this defines the macro that w

use core::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use core::iter::Iterator;
use core::fmt;
use core::convert::TryFrom;

pub struct Elements<T> {
    tuple:  T,
    index:  usize
}

pub trait TupleElements {
    type Element;
    const N: usize;
    
    /// returns an Iterator over the elements of the tuple
    fn elements(&self) -> Elements<&Self>;
    
    /// attempt to access the n-th element
    fn get(&self, n: usize) -> Option<&Self::Element>;
    
    /// attempt to access the n-th element mutablbly.
    fn get_mut(&mut self, n: usize) -> Option<&mut Self::Element>;
}

impl<'a, T> Iterator for Elements<&'a T> where T: TupleElements {
    type Item = &'a T::Element;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.tuple.get(self.index);
        if let Some(_) = t {
            self.index += 1;
        }
        t
    }
}

macro_rules! A { ($a:ident, $b:ident) => ($a) }
macro_rules! a { ($a:expr, $b:expr) => ($a) }
macro_rules! Rev {
    (@ $a:ident; $(  $b:ident),*                 ) => (     ($a $(,$b) *              ) );
    (@ $a:ident  $(, $b:ident) * ; $($c:ident),* ) => ( Rev!(@  $( $b),* ; $a $(,$c)* ) );
    (  $a:ident                                  ) => (      $a                         );
    (  $a:ident  $(, $b:ident) *                 ) => ( Rev!(@  $( $b),* ; $a         ) );
}
macro_rules! rev {
    (@ $a:expr; $(  $b:expr),*                ) => (     ($a $(,$b) *              ) );
    (@ $a:expr  $(, $b:expr) * ; $($c:expr),* ) => ( rev!(@  $( $b),* ; $a $(,$c)* ) );
    (  $a:expr                                ) => (      $a                         );
    (  $a:expr  $(, $b:expr) *                ) => ( rev!(@  $( $b),* ; $a         ) );
}

#[macro_export]
macro_rules! impl_tuple {
    ($def:ident) => ($def!(
    T1 { 0 -> A }
    T2 { 0 -> A, 1 -> B }
    T3 { 0 -> A, 1 -> B, 2 -> C }
    T4 { 0 -> A, 1 -> B, 2 -> C, 3 -> D }
    T5 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E }
    T6 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, 5 -> F }
    T7 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, 5 -> F, 6 -> G }
    T8 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, 5 -> F, 6 -> G, 7 -> H }
    );)
}

#[macro_use]
mod m_init;
impl_tuple!(m_init);

#[cfg(impl_num)]
#[macro_use]
mod m_num;
#[cfg(impl_num)]
impl_tuple!(m_num);

#[macro_use]
mod m_tuple;
impl_tuple!(m_tuple);

/*
use itertools::tuple_impl::TupleCollect;
#[macro_use]
mod impl_itertools;
trace_macros!(true);
impl_tuple!(impl_itertools);
*/
