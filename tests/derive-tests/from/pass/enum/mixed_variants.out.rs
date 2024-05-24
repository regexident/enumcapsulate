use enumcapsulate::derive::From;
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
