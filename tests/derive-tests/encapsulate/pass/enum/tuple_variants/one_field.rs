use enumcapsulate::{AsVariant, AsVariantMut, AsVariantRef, Encapsulate, FromVariant, IntoVariant};

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;

#[derive(Encapsulate)]
pub enum Enum {
    VariantA(VariantA),
    VariantB { b: VariantB },
}

fn check<T, U>()
where
    T: FromVariant<U>
        + IntoVariant<U>
        + From<U>
        + TryInto<U>
        + AsVariant<U>
        + AsVariantRef<U>
        + AsVariantMut<U>,
{
}

fn main() {
    check::<Enum, VariantA>();
    check::<Enum, VariantB>();
}
