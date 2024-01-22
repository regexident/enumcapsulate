use enumcapsulate::{derive::Encapsulate, AsVariantMut, AsVariantRef, IntoVariant};

pub struct VariantA;
pub struct VariantB;

#[derive(Encapsulate)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn check<T, U>()
where
    T: AsVariantRef<U> + AsVariantMut<U> + IntoVariant<U> + From<U> + TryInto<U>,
{
}

fn main() {
    check::<Enum, VariantA>();
    check::<Enum, VariantB>();
}
