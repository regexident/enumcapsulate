use enumcapsulate::derive::VariantDiscriminant;
use enumcapsulate::VariantDiscriminant;

pub struct VariantA;
pub struct VariantB;

#[derive(VariantDiscriminant)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField {
        variant_a: VariantA,
    },
    TwoTupleFields(VariantA, VariantB),
    TwoStructFields {
        variant_a: VariantA,
        variant_b: VariantB,
    },
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
