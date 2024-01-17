use enumcapsulate::{
    derive::Encapsulate, AsVariantMut, AsVariantRef, IntoVariant, VariantDowncast,
};

#[derive(Encapsulate)]
pub enum Enum {}

fn check<T>()
where
    T: VariantDowncast,
{
}

fn main() {
    check::<Enum>();
}
