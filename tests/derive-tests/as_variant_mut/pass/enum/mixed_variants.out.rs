use enumcapsulate::derive::AsVariantMut;
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
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::OneTupleField(inner) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::OneStructField { variant: inner } => Some(inner),
            _ => None,
        }
    }
}
fn main() {
    use enumcapsulate::AsVariantMut;
    let mut subject = Enum::Unit;
    {
        let _: Option<&mut VariantA> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_mut();
    }
}
