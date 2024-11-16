use enumcapsulate::{AsVariant, AsVariantMut, AsVariantRef, Encapsulate, IntoVariant};

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
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
    T: AsVariant<U> + AsVariantRef<U> + AsVariantMut<U> + IntoVariant<U> + From<U> + TryInto<U>,
{
}

fn main() {
    check::<Enum, VariantA>();
    check::<Enum, VariantB>();
}
