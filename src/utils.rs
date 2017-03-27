macro_rules! A { ($a:ident, $b:ident) => ($a) }
macro_rules! a { ($a:expr, $b:expr) => ($a) }
macro_rules! Rev {
    (@ $mac:ident $(,$args:ident)* ; $a:ident                ; $($c:ident),* ) => (        $mac!($($args),* ; $a           $(,$c)* ) );
    (@ $mac:ident $(,$args:ident)* ; $a:ident  $(,$b:ident)* ; $($c:ident),* ) => ( Rev!(@ $mac $(,$args)*  ; $($b),* ; $a $(,$c)* ) );
    (  $mac:ident $(,$args:ident)* ; $a:ident,                               ) => (        $mac!($($args),* ; $a                   ) );
    (  $mac:ident $(,$args:ident)* ; $a:ident, $($b:ident,)*                 ) => ( Rev!(@ $mac $(,$args)*  ; $($b),* ; $a         ) );
}
macro_rules! rev {
    (@ $mac:ident $(,$args:ident)* ; $a:expr               ; $($c:expr),* ) => (        $mac!($($args),* ; $a           $(,$c)* ) );
    (@ $mac:ident $(,$args:ident)* ; $a:expr  $(,$b:expr)* ; $($c:expr),* ) => ( rev!(@ $mac $(,$args)*  ; $($b),* ; $a $(,$c)* ) );
    (  $mac:ident $(,$args:ident)* ; $a:expr,                             ) => (        $mac!($($args),* ; $a                   ) );
    (  $mac:ident $(,$args:ident)* ; $a:expr, $($b:expr,)*                ) => ( rev!(@ $mac $(,$args)*  ; $($b),* ; $a         ) );
}

macro_rules! Rot_l {
    ( $mac:ident $(,$args:ident)* ; $a:ident, $($b:ident,)* ) => ( $mac!($($args),* ; $($b,)* $a ) )
}
macro_rules! rot_l {
    ( $mac:ident $(,$args:ident)* ; $a:expr, $($b:expr,)* ) => ( $mac!($($args),* ; $($b,)* $a ) )
}
macro_rules! Rot_r {
    (@ $mac:ident $(,$args:ident)* ; $a:ident                ; $($c:ident),* ) => (          $mac!($($args),* ; $a $(,$c)*           ) );
    (@ $mac:ident $(,$args:ident)* ; $a:ident, $($b:ident),* ; $($c:ident),* ) => ( Rot_r!(@ $mac $(,$args)*  ; $($b),* ; $($c,)* $a ) );
    (  $mac:ident $(,$args:ident)* ; $a:ident,                               ) => (          $mac!($($args),* ; $a                   ) );
    (  $mac:ident $(,$args:ident)* ; $a:ident, $($b:ident,)*                 ) => ( Rot_r!(@ $mac $(,$args)*  ; $($b),* ; $a         ) );
}
macro_rules! rot_r {
    (@ $mac:ident $(,$args:ident)* ; $a:expr               ; $($c:expr),* ) => (          $mac!($($args),* ; $a $(,$c)*           ) );
    (@ $mac:ident $(,$args:ident)* ; $a:expr, $($b:expr),* ; $($c:expr),* ) => ( rot_r!(@ $mac $(,$args)*  ; $($b),* ; $($c,)* $a ) );
    (  $mac:ident $(,$args:ident)* ; $a:expr,                             ) => (          $mac!($($args),* ; $a                   ) );
    (  $mac:ident $(,$args:ident)* ; $a:expr, $($b:expr,)*                ) => ( rot_r!(@ $mac $(,$args)*  ; $($b),* ; $a         ) );
}

macro_rules! x_ty_ident {
    ( $ty:ident; $($args:ident),* ) => ( $ty<$($args),*> )
}
macro_rules! x_ty_expr {
    ( $ty:ident; $($args:expr),* ) => ( $ty($($args),*) )
}
macro_rules! x_tuple_ident {
    ( ; $($args:ident),* ) => ( ($($args),*) )
}
macro_rules! x_tuple_expr {
    ( ; $($args:expr),* ) => ( ($($args),*) )
}

