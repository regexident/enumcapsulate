use enumcapsulate::From;

pub struct VariantA;
pub struct VariantB;

#[derive(From)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
}

fn main() {
    let _ = Enum::from(VariantA);
    let _ = Enum::from(VariantB);
}
