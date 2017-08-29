# tuple
Element wise operations on tuples!

# [Documentation](https://docs.rs/tuple)

This crate allows to generalize operations to tuples using macros.
Please look at the Documentation for examples.

Supported Operations so far:
 - `TupleElements` trait (for tuples where all elements have the same type)
   - `get` and `get_mut` functions
   - `N` (number of elements)
 - `ops`
   - `Index`,`IndexMut`
   - `Clone`, `Copy`
   - `Add`, `AddAssign`
   - `Sub`, `SubAssign`
   - `Mul`, `Div`
   - `Neg`
 - `fmt::Debug`
 - `iter`
   - `Iterator` Iterate over all elements at once (assuming they implement `Iterator`).
     The iterations stops once one or more elements return `None`.
 - `convert`
   - `From` and `Into` their tuple equivalent.
   - `From` and `Into` their array equivalent.
   - `TryFrom` slice
 - `Map<T>` trait
    - `map`: apply a function to each element and return the resulting tuple
 - other
   - `rev`: reverse the elements
   - `rot_l` and `rot_r`: rotate elements
   - `split`: make two separate tuples out of one
   - `join`: turn two tuples into one

