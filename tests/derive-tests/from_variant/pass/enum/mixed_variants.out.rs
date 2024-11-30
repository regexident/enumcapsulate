use enumcapsulate::FromVariant;
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
    #[enumcapsulate(field = 1)]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(field = "variant")]
    IncludedStruct { value: u8, variant: VariantD },
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
impl ::enumcapsulate::FromVariant<VariantC> for Enum {
    fn from_variant(inner: VariantC) -> Self {
        Self::IncludedTuple(Default::default(), inner)
    }
}
impl ::enumcapsulate::FromVariant<VariantD> for Enum {
    fn from_variant(inner: VariantD) -> Self {
        Self::IncludedStruct {
            value: Default::default(),
            variant: inner,
        }
    }
}
fn main() {
    let _ = Enum::from_variant(VariantA);
    let _ = Enum::from_variant(VariantB);
}
