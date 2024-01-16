use enumcapsulate::derive::AsVariantRef;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::enumcapsulate::AsVariantRef<VariantA> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::VariantA(variant) => Some(variant),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantB> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantB> {
        match self {
            Enum::VariantB(variant) => Some(variant),
            _ => None,
        }
    }
}
fn main() {
    use enumcapsulate::AsVariantRef;
    let subject = Enum::VariantA(VariantA);
    let _: Option<&VariantA> = subject.as_variant_ref();
    let _: Option<&VariantB> = subject.as_variant_ref();
}
