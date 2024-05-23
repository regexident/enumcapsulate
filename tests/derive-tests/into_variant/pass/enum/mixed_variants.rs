use enumcapsulate::derive::IntoVariant;

pub struct VariantA;
pub struct VariantB;

#[derive(IntoVariant)]
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
    use enumcapsulate::IntoVariant;

    {
        let subject = Enum::Unit;
        let _: Result<VariantA, Enum> = subject.into_variant();
    }
    {
        let subject = Enum::Unit;
        let _: Result<VariantB, Enum> = subject.into_variant();
    }
}
