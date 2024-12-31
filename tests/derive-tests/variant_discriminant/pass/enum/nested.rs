use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
pub enum VariantC {
    VariantC1,
    VariantC2,
}

#[derive(VariantDiscriminant)]
pub enum VariantD {
    VariantD1,
    VariantD2,
}

#[derive(VariantDiscriminant)]
pub enum VariantE {
    VariantE1,
    VariantE2,
}

#[derive(VariantDiscriminant)]
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum {
    VariantA,
    #[enumcapsulate(discriminant(value = 42))]
    VariantB,
    #[enumcapsulate(discriminant(nested = VariantCDiscriminant))]
    VariantC {
        c: VariantC,
    },
    #[enumcapsulate(discriminant(name = RenamedVariant, nested = VariantDDiscriminant))]
    VariantD(VariantD),
    #[enumcapsulate(field = 1, discriminant(nested = VariantEDiscriminant))]
    VariantE(bool, VariantE),
}

fn main() {}
