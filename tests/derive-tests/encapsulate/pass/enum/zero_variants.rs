use enumcapsulate::{AsVariantMut, AsVariantRef, Encapsulate, IntoVariant, VariantDowncast};

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
