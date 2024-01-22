use enumcapsulate::derive::From;

pub struct VariantA;
pub struct VariantB;

#[derive(From)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn main() {
    let _ = Enum::from(VariantA);
    let _ = Enum::from(VariantB);
}
