use enumcapsulate::derive::IsVariant;
use enumcapsulate::IsVariant;

pub struct VariantA;
pub struct VariantB;

#[derive(IsVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn check<T>()
where
    T: IsVariant,
{
}

fn main() {
    check::<Enum>();
    check::<Enum>();
}
