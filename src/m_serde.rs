use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de::{SeqAccess, Visitor, Deserializer, Deserialize, Error};
use super::*;
use core::marker::PhantomData;

macro_rules! impl_serde {
    ($($Tuple:ident $Arr:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        impl<$($T),*> Serialize for $Tuple<$($T),*> where $( $T: Serialize ),* {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
            {
                let len = $(a!(1, $idx)+)* 0;
                let mut tuple = serializer.serialize_tuple(len)?;
             $( tuple.serialize_element(&self.$idx)?; )*
                tuple.end()
            }
        }
        impl<'de, $($T),*> Deserialize<'de> for $Tuple<$($T),*> where $( $T: Deserialize<'de> ),* {
            fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
            where De: Deserializer<'de>
            {
                const LEN: usize = $(a!(1, $idx)+)* 0;
                
                struct TupleVisitor<$($t),*>(PhantomData<($($t,)*)>);
                impl<'df, $($t: Deserialize<'df>),*> Visitor<'df> for TupleVisitor<$($t),*> {
                    type Value = $Tuple<$($t),*>;
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        write!(formatter, "Tuple of length {}", LEN)
                    }
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where A: SeqAccess<'df>
                    {
                     $( let $t = seq.next_element()?; )*
                        
                        match ($($t,)* seq.next_element::<()>()?) {
                            ($(Some($t),)* None) => Ok($Tuple($($t),*)),
                            _ => Err(A::Error::invalid_length(LEN, &self))
                        }
                    }
                }
                
                deserializer.deserialize_map(TupleVisitor(PhantomData))
            }
        }
    )*)
}

impl_tuple!(impl_serde);
