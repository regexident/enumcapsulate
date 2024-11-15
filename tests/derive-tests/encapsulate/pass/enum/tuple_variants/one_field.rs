use enumcapsulate::{AsVariantMut, AsVariantRef, Encapsulate, IntoVariant};

pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;

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
