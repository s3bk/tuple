/*!
# Examples

```
extern crate tuple;
use tuple::*;
# fn main() {}
```
All following operations are defined on the `T1` .. `Tn` type of this crate,
as well for the normal tuple types.

## Element-wise operations

```
# extern crate tuple;
# use tuple::*;
# fn main() {
let a = T2(3, 4) + T2(5, 4);
assert_eq!(a, T2(8, 8));

let b = T2(3u32, 4.0f32) * T2(7, 3.0);
assert_eq!(b, T2(21, 12.));
# }
```

## Indexing

This is implemented in the [`TupleElements`](trait.TupleElements.html) trait.

Indexing works as expected and panics when out of bounds.
There are also `get` and `get_mut` functions that return `Option<&T>` and `Option<&mut T>`.

```
# extern crate tuple;
# use tuple::*;
# fn main() {
assert_eq!(T3(1, 2, 3)[2], 3);

assert_eq!(T2(7, 8).get(1), Some(&8));
assert_eq!(T2(7, 8).get(2), None);
# }
```

## Iterate over the elements of a tuple
```
# extern crate tuple;
# use tuple::*;
# fn main() {
for i in T2(1, 2).elements() {
    println!("{}", i);
}

let mut b = T3(3, 4, 5);
for i in b.elements_mut() {
    *i += 1;
}
assert_eq!(b.elements().sum::<u32>(), 15);
# }
```

## Joining two tuples
```
# extern crate tuple;
# use tuple::*;
# fn main() {
let a = T2(1, 2);
let b = T3(3, 4, 5);
assert_eq!(a.join(b), T5(1, 2, 3, 4, 5));
# }
```

## Splitting a tuple in two parts
```
# extern crate tuple;
# use tuple::*;
# fn main() {
let a = T4(1, 2, 3, 4);
let (b, c): (T1<_>, _) = a.split(); // split needs a type hint for the left side
assert_eq!(b, T1(1));
assert_eq!(c, T3(2, 3, 4));
# }
```

## Rotate and Reverse

```
# extern crate tuple;
# use tuple::*;
# fn main() {
let a = T4((), 2, 3, true);
assert_eq!(a.rot_l(),   T4(2, 3, true, ())); // rotate left
assert_eq!(a.rot_r(),   T4(true, (), 2, 3)); // rotate right
assert_eq!(a.reverse(), T4(true, 3, 2, ())); // reverse
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

# fn main() {}
```
**/
#![feature(associated_consts)]
#![feature(trace_macros)]
#![no_std]
#![allow(non_camel_case_types)]

#[cfg(feature="impl_num")]
extern crate num_traits;
#[cfg(feature="impl_num")]
use num_traits as num;
//extern crate itertools;


// this defines the macro that w

use core::ops;
use core::iter::Iterator;
use core::fmt;

pub struct Elements<T> {
    tuple:  T,
    index:  usize
}

pub trait TupleElements {
    type Element;
    const N: usize;
    
    /// returns an Iterator over references to the elements of the tuple
    fn elements(&self) -> Elements<&Self>;
    
    /// returns an Iterator over mutable references to elements of the tuple
    fn elements_mut(&mut self) -> Elements<&mut Self>;
    
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
impl<'a, T> Iterator for Elements<&'a mut T> where T: TupleElements {
    type Item = &'a mut T::Element;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.tuple.get_mut(self.index) {
            self.index += 1;
            
            // we only hand out one reference to each item
            // and that lifetime is limited to the Elements struct
            Some(unsafe { &mut *(t as *mut T::Element) })
        } else { 
            None
        }
    }
}

/// Allows to join/concatenate two tuples
pub trait OpJoin<RHS> {
    type Output;
    fn join(self, rhs: RHS) -> Self::Output;
}

pub trait OpSplit<L> {
    type R;
    fn split(self) -> (L, Self::R);
}

pub trait OpRotateLeft {
    type Output;
    /// rotate left. The previously first element is now the first.
    fn rot_l(self) -> Self::Output;
}
pub trait OpRotateRight {
    type Output;
    /// rotate right. The previously last element is now the last.
    fn rot_r(self) -> Self::Output;
}
pub trait OpReverse {
    type Output;
    /// reverse the elements.
    fn reverse(self) -> Self::Output;
}

#[macro_use]
mod utils;

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

#[macro_use]
mod m_ops;
impl_tuple!(m_ops);

#[cfg(feature="impl_num")]
#[macro_use]
mod m_num;
#[cfg(feature="impl_num")]
impl_tuple!(m_num);

#[macro_use]
mod m_tuple;
impl_tuple!(m_tuple);
m_join!();

/*
use itertools::tuple_impl::TupleCollect;
#[macro_use]
mod impl_itertools;
trace_macros!(true);
impl_tuple!(impl_itertools);
*/
