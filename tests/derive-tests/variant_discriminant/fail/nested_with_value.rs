use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
pub enum VariantA {
    VariantA1,
    VariantA2,
}

#[derive(VariantDiscriminant)]
pub enum Enum {
    #[enumcapsulate(discriminant(nested = VariantADiscriminant, value = 42))]
    VariantA(VariantA),
}

fn main() {}
