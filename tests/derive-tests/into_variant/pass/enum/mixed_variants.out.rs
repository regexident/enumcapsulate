use enumcapsulate::derive::IntoVariant;
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
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::OneTupleField(inner) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::OneStructField { variant: inner } => Ok(inner),
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
