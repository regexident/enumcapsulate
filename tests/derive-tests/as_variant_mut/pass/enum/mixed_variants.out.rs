use enumcapsulate::AsVariantMut;
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
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::OneTupleField(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::OneStructField { variant: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantC> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantC> {
        match self {
            Enum::IncludedTuple(_, inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantD> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantD> {
        match self {
            Enum::IncludedStruct { variant: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
fn main() {
    let mut subject = Enum::Unit;
    {
        let _: Option<&mut VariantA> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_mut();
    }
}
