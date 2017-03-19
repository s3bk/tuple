/*!
# Examples

```
extern crate tuple;
use tuple::*;
# fn main() {
let a = T2(3, 4) + T2(5, 4);
assert_eq!(a, T2(8, 8));

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

#[cfg(impl_num)]
extern crate num_traits;

#[macro_use]
mod macros;

// this defines the macro that w

use std::ops::{Add, Sub, Mul, Div};
use std::iter::Iterator;
use std::fmt;

impl_tuple!(tuple_init);

#[cfg(impl_num)]
#[macro_use]
mod impl_num;
#[cfg(impl_num)]
impl_tuple!(impl_num);
