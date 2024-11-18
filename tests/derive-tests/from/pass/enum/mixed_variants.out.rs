use enumcapsulate::From;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
}
impl ::core::convert::From<VariantA> for Enum {
    fn from(inner: VariantA) -> Self {
        Self::OneTupleField(inner)
    }
}
impl ::core::convert::From<VariantB> for Enum {
    fn from(inner: VariantB) -> Self {
        Self::OneStructField {
            variant: inner,
        }
    }
}
fn main() {
    let _ = Enum::from(VariantA);
    let _ = Enum::from(VariantB);
}
