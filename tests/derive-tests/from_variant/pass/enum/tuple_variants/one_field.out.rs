use enumcapsulate::derive::FromVariant;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::enumcapsulate::FromVariant<VariantA> for Enum {
    fn from_variant(inner: VariantA) -> Self {
        Self::VariantA(inner)
    }
}
impl ::enumcapsulate::FromVariant<VariantB> for Enum {
    fn from_variant(inner: VariantB) -> Self {
        Self::VariantB(inner)
    }
}
fn main() {
    use enumcapsulate::FromVariant;
    let _ = Enum::from_variant(VariantA);
    let _ = Enum::from_variant(VariantB);
}
