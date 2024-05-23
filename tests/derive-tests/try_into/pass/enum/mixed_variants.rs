use enumcapsulate::derive::TryInto;

pub struct VariantA;
pub struct VariantB;

#[derive(TryInto)]
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
    {
        let subject = Enum::Unit;
        let _: Result<VariantA, Enum> = subject.try_into();
    }
    {
        let subject = Enum::Unit;
        let _: Result<VariantB, Enum> = subject.try_into();
    }
}
