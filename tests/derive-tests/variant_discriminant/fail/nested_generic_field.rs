use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
pub enum VariantA<T> {
    Variant(T),
}

#[derive(VariantDiscriminant)]
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum<T> {
    #[enumcapsulate(discriminant(nested))]
    VariantA(VariantA<T>),
}

fn main() {}
