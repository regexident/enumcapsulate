use enumcapsulate::derive::VariantDiscriminant;
use enumcapsulate::VariantDiscriminant;

pub struct VariantA;
pub struct VariantB;

#[derive(VariantDiscriminant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn check<T>()
where
    T: VariantDiscriminant,
{
}

fn main() {
    check::<Enum>();
    check::<Enum>();
}
