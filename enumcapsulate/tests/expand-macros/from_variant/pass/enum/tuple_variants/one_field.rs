use enumcapsulate::derive::FromVariant;

pub struct VariantA;
pub struct VariantB;

#[derive(FromVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn main() {
    use enumcapsulate::FromVariant;

    let _ = Enum::from_variant(VariantA);
    let _ = Enum::from_variant(VariantB);
}
