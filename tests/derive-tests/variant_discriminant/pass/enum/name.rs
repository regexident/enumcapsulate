use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
#[enumcapsulate(discriminant(name = RenamedDiscriminant))]
pub enum Enum {
    VariantA,
    VariantB,
    VariantC,
    #[enumcapsulate(discriminant(name = RenamedVariant))]
    VariantD,
}

fn main() {}
