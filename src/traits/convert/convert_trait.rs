use super::dot::*;

pub trait Convert: FromDOT + IntoDOT {}

impl<T> Convert for T where T: FromDOT + IntoDOT {}

#[macro_export]
macro_rules! impl_convert_trait {
    ($graph:ident) => {
        impl<T> $crate::traits::convert::FromDOT for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            type Error = $crate::errors::Error<T>;

            fn from_dot(value: &String) -> Result<Self, Self::Error> {
                todo!();
            }
        }

        impl<T> $crate::traits::convert::IntoDOT for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            type Error = $crate::errors::Error<T>;

            fn into_dot(&self) -> String {
                todo!();
            }
        }
    };
}
