use enumcapsulate::derive::IntoVariant;
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
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(bool),
    #[enumcapsulate(include(field = 1))]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(include(field = "variant"))]
    IncludedStruct { value: u8, variant: VariantD },
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::OneTupleField(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::OneStructField { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantC> for Enum {
    fn into_variant(self) -> Result<VariantC, Self> {
        match self {
            Enum::IncludedTuple(_, inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantD> for Enum {
    fn into_variant(self) -> Result<VariantD, Self> {
        match self {
            Enum::IncludedStruct { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
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
