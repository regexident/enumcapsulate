use enumcapsulate::VariantDiscriminant;
pub enum Enum {
    #[enumcapsulate(discriminant(nested, value = 42))]
    VariantA,
}
fn main() {}
