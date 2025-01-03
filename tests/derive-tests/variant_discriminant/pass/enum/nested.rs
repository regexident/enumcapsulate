use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
pub enum VariantC {
    Variant,
}

#[derive(VariantDiscriminant)]
pub enum VariantD {
    Variant,
}

#[derive(VariantDiscriminant)]
pub enum VariantE<T> {
    Variant(T),
}

#[derive(VariantDiscriminant)]
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum<T> {
    VariantA,
    #[enumcapsulate(discriminant(value = 42))]
    VariantB,
    #[enumcapsulate(discriminant(nested))]
    VariantC {
        c: VariantC,
    },
    #[enumcapsulate(discriminant(name = RenamedVariant, nested))]
    VariantD(VariantD),
    #[enumcapsulate(field = 1, discriminant(nested = VariantEDiscriminant))]
    VariantE(bool, VariantE<T>),
}

fn main() {}
