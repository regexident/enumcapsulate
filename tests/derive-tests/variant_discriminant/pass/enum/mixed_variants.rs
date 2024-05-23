use enumcapsulate::derive::VariantDiscriminant;

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

fn main() {
    use enumcapsulate::VariantDiscriminant;

    fn check<T>()
    where
        T: VariantDiscriminant,
    {
    }

    check::<Enum>();
}
