use enumcapsulate::derive::IntoVariant;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::VariantA(variant) => Ok(variant),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::VariantB(variant) => Ok(variant),
            err => Err(err),
        }
    }
}
fn main() {
    use enumcapsulate::IntoVariant;
    let subject = Enum::VariantA(VariantA);
    let _: Result<VariantA, Enum> = subject.into_variant();
}
