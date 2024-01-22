use enumcapsulate::derive::TryInto;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::core::convert::TryFrom<Enum> for VariantA {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::VariantA(inner) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantB {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::VariantB(inner) => Ok(inner),
            err => Err(err),
        }
    }
}
fn main() {
    let subject = Enum::VariantA(VariantA);
    let _: Result<VariantA, Enum> = subject.try_into();
}
