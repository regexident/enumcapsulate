use enumcapsulate::derive::TryInto;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(bool),
}
impl ::core::convert::TryFrom<Enum> for VariantA {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::OneTupleField(inner) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantB {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::OneStructField { variant: inner } => Ok(inner),
            err => Err(err),
        }
    }
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
