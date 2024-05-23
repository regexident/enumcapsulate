use enumcapsulate::derive::From;

pub struct VariantA;
pub struct VariantB;

#[derive(From)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
}

fn main() {
    let _ = Enum::from(VariantA);
    let _ = Enum::from(VariantB);
}
