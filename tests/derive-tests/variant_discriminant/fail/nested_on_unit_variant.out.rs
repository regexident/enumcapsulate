use enumcapsulate::VariantDiscriminant;
pub enum Enum {
    #[enumcapsulate(discriminant(nested = VariantADiscriminant, value = 42))]
    VariantA,
}
fn main() {}
