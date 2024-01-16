use enumcapsulate::derive::AsVariantMut;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::VariantA(variant) => Some(variant),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::VariantB(variant) => Some(variant),
            _ => None,
        }
    }
}
fn main() {
    use enumcapsulate::AsVariantMut;
    let mut subject = Enum::VariantA(VariantA);
    {
        let _: Option<&mut VariantA> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_mut();
    }
}
