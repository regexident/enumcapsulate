use enumcapsulate::derive::AsVariantRef;
pub struct VariantA;
#[automatically_derived]
impl ::core::clone::Clone for VariantA {
    #[inline]
    fn clone(&self) -> VariantA {
        VariantA
    }
}
pub struct VariantB;
#[automatically_derived]
impl ::core::clone::Clone for VariantB {
    #[inline]
    fn clone(&self) -> VariantB {
        VariantB
    }
}
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
}
impl ::enumcapsulate::AsVariantRef<VariantA> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::OneTupleField(inner) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantB> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantB> {
        match self {
            Enum::OneStructField { variant: inner } => Some(inner),
            _ => None,
        }
    }
}
fn main() {
    use enumcapsulate::AsVariantRef;
    let subject = Enum::Unit;
    let _: Option<&VariantA> = subject.as_variant_ref();
    let _: Option<&VariantB> = subject.as_variant_ref();
    let _: Option<VariantA> = subject.as_variant();
    let _: Option<VariantB> = subject.as_variant();
}
