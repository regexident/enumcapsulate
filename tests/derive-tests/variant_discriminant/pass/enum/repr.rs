use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum {
    VariantA,
    VariantB = 5,
    VariantC,
    #[enumcapsulate(discriminant(value = 42))]
    VariantD,
}

fn main() {}
