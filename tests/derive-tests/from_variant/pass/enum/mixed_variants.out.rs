use enumcapsulate::derive::FromVariant;
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
impl ::enumcapsulate::FromVariant<VariantA> for Enum {
    fn from_variant(inner: VariantA) -> Self {
        Self::OneTupleField(inner)
    }
}
impl ::enumcapsulate::FromVariant<VariantB> for Enum {
    fn from_variant(inner: VariantB) -> Self {
        Self::OneStructField {
            variant: inner,
        }
    }
}
fn main() {
    use enumcapsulate::FromVariant;
    let _ = Enum::from_variant(VariantA);
    let _ = Enum::from_variant(VariantB);
}
