use enumcapsulate::{derive::AsVariant, AsVariantMut, AsVariantRef};

pub struct VariantA;
pub struct VariantB;

#[derive(AsVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn check<T, U>()
where
    T: AsVariantRef<U> + AsVariantMut<U>,
{
}

fn main() {
    check::<Enum, VariantA>();
    check::<Enum, VariantB>();
}
