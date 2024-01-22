use enumcapsulate::derive::From;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::core::convert::From<VariantA> for Enum {
    fn from(inner: VariantA) -> Self {
        Self::VariantA(inner)
    }
}
impl ::core::convert::From<VariantB> for Enum {
    fn from(inner: VariantB) -> Self {
        Self::VariantB(inner)
    }
}
fn main() {
    let _ = Enum::from(VariantA);
    let _ = Enum::from(VariantB);
}
