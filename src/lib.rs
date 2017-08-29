/*!

# Feature flags
 - `impl_num` add support for traits from the num crate. (default on)
 - `impl_simd` add support for simd types. (default off)
 - `impl_serde` impl Serialize and Deserialize from serde

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

assert_eq!(T3(1, 2, 3).map(|x| x * 2), T3(2, 4, 6));
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

### Consume a tuple and iterate over the elements
```
# extern crate tuple;
# use tuple::*;
# fn main() {
for i in T2(String::from("hello"), String::from("world")).into_elements() {
    let s: String = i; // it's really a String
    println!("{}", s);
}
# }
```

## Conversions

```
# #![feature(try_from)]
# extern crate tuple;
# use tuple::*;
# fn main() {
use std::convert::TryFrom;
// slice to tuple
assert_eq!(T3::try_from(&[1u8, 2, 3, 4, 5][..]), Ok(T3(1, 2, 3)));

// tuple to and from array
let t = T3(1, 2, 3);
let a: [u8; 3] = t.into();
let t: T3<_, _, _> = a.into();
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
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(

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

#![feature(trace_macros)]
#![feature(try_from)] 
#![feature(slice_patterns)]
#![feature(cfg_target_feature)]
#![no_std]
#![allow(non_camel_case_types, non_snake_case)]

#[cfg(feature="impl_num")]
extern crate num_traits;
#[cfg(feature="impl_num")]
use num_traits as num;
//extern crate itertools;

#[cfg(feature="impl_simd")]
extern crate simd;

#[cfg(feature="impl_serde")]
extern crate serde;

use core::{ptr, mem, fmt};

pub struct Elements<T> {
    tuple:  T,
    index:  usize
}
impl<T> Elements<T> {
    fn new(t: T) -> Elements<T> {
        Elements { tuple: t, index: 0 }
    }
}
pub struct IntoElements<T: TupleElements> {
    tuple:  Option<T>,
    index:  usize
}
impl<T: TupleElements> IntoElements<T> {
    fn new(t: T) -> IntoElements<T> {
        IntoElements { tuple: Some(t), index: 0 }
    }
}

/// This trais is marked as unsafe, due to the requirement of the get_mut method,
/// which is required work as an injective map of `index -> element`
///
/// A tuple must not have a `Drop` implementation.
pub unsafe trait TupleElements: Sized {
    type Element;
    const N: usize;
    
    /// returns an Iterator over references to the elements of the tuple
    fn elements(&self) -> Elements<&Self> { Elements::new(self) }
    
    /// returns an Iterator over mutable references to elements of the tuple
    fn elements_mut(&mut self) -> Elements<&mut Self> { Elements::new(self) }
    
    // return an Iterator over the elements of the tuple
    fn into_elements(self) -> IntoElements<Self> { IntoElements::new(self) }
    
    /// attempt to access the n-th element
    fn get(&self, n: usize) -> Option<&Self::Element>;
    
    /// attempt to access the n-th element mutablbly.
    /// This function shall not return the same data for two different indices.
    fn get_mut(&mut self, n: usize) -> Option<&mut Self::Element>;
    
    fn from_iter<I>(iter: I) -> Option<Self> where I: Iterator<Item=Self::Element>;


}

pub trait Map<T>: TupleElements {
    type Output: TupleElements;
    fn map<F>(self, f: F) -> Self::Output where F: Fn(Self::Element) -> <Self::Output as TupleElements>::Element;
}

/**
splat: copy the argument into all elements

```
# extern crate tuple;
# use tuple::*;
# fn main() {
let a = T4::splat(42);
assert_eq!(a,   T4(42, 42, 42, 42));
# }
```
*/
pub trait Splat<T> {
    fn splat(T) -> Self;
}


#[derive(Debug, Eq, PartialEq)]
pub enum ConvertError {
    OutOfBounds
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
impl<T> Iterator for IntoElements<T> where T: TupleElements {
    type Item = T::Element;
    fn next(&mut self) -> Option<Self::Item> {
        match self.tuple.as_mut().unwrap().get(self.index) {
            Some(p) => {
                self.index += 1; // mark as taken
                let v = unsafe { ptr::read(p) }; // read it
                Some(v)
            },
            None => None
        }
    }
}
impl<T> Drop for IntoElements<T> where T: TupleElements {
    fn drop(&mut self) {
        let mut tuple = self.tuple.take().unwrap();
        // only drop remaining elements
        for i in self.index .. T::N {
            unsafe {
                ptr::drop_in_place(tuple.get_mut(i).unwrap());
            }
        }
        mem::forget(tuple);
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
/*  python3:
import string
for i in range(1, 17):
    print("T{i} {{ {inner} }}".format(i=i, inner=", ".join("{a}.{b}.{n}".format(a=string.ascii_uppercase[j],b=string.ascii_lowercase[j],n=j) for j in range(i))))
*/
#[macro_export]
macro_rules! impl_tuple {
    ($def:ident) => ($def!(
T1 { A.a.0 }
T2 { A.a.0, B.b.1 }
T3 { A.a.0, B.b.1, C.c.2 }
T4 { A.a.0, B.b.1, C.c.2, D.d.3 }
T5 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4 }
T6 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5 }
T7 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6 }
T8 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7 }
T9 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8 }
T10 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9 }
T11 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9, K.k.10 }
T12 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9, K.k.10, L.l.11 }
T13 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9, K.k.10, L.l.11, M.m.12 }
T14 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9, K.k.10, L.l.11, M.m.12, N.n.13 }
T15 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9, K.k.10, L.l.11, M.m.12, N.n.13, O.o.14 }
T16 { A.a.0, B.b.1, C.c.2, D.d.3, E.e.4, F.f.5, G.g.6, H.h.7, I.i.8, J.j.9, K.k.10, L.l.11, M.m.12, N.n.13, O.o.14, P.p.15 }
    );)
}
macro_rules! init {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        pub struct $Tuple<$($T),*>($(pub $T),*);
    )*)
}
impl_tuple!(init);

mod m_init;
mod m_ops;
mod m_convert;
mod m_array;

#[cfg(feature="impl_num")]
mod m_num;

mod m_tuple;

#[cfg(all(feature="impl_simd", any(target_arch="x86", target_arch="x86_64")))]
#[macro_use]
mod m_simd;

//#[cfg(feature="impl_simd")]
//impl_tuple!(m_simd);

#[cfg(feature="impl_serde")]
mod m_serde;

/*
use itertools::tuple_impl::TupleCollect;
#[macro_use]
mod impl_itertools;
trace_macros!(true);
impl_tuple!(impl_itertools);
*/

/** 
```
# extern crate tuple;
# use tuple::*;
# fn main() {
assert_eq!(tuple("hello world".split(' ')), Some(("hello", "world")));
# }
```
**/
pub fn tuple<T, I>(iter: I) -> Option<T> where
    T: TupleElements, I: Iterator<Item=T::Element>
{
    T::from_iter(iter)
}
