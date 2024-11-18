use enumcapsulate::TryInto;
pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    #[enumcapsulate(exclude)]
    TwoTupleFields(i32, u32),
    #[enumcapsulate(exclude)]
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
    #[enumcapsulate(field(index = 1))]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(field(name = "variant"))]
    IncludedStruct { value: u8, variant: VariantD },
}
impl ::core::convert::TryFrom<Enum> for VariantA {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::OneTupleField(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantB {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::OneStructField { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantC {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::IncludedTuple(_, inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantD {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::IncludedStruct { variant: inner, .. } => Ok(inner),
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
