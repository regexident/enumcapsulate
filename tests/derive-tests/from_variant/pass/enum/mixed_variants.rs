use enumcapsulate::derive::FromVariant;

pub struct VariantA;
pub struct VariantB;

#[derive(FromVariant)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    TwoTupleFields(i32, u32),
    TwoStructFields {
        a: i32,
        b: u32,
    },
}

fn main() {
    use enumcapsulate::FromVariant;

    let _ = Enum::from_variant(VariantA);
    let _ = Enum::from_variant(VariantB);
}
