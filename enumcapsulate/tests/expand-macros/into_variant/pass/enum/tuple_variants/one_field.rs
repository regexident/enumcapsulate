use enumcapsulate::derive::IntoVariant;

pub struct VariantA;
pub struct VariantB;

#[derive(IntoVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn main() {
    use enumcapsulate::IntoVariant;

    let subject = Enum::VariantA(VariantA);

    let _: Result<VariantA, Enum> = subject.into_variant();
}
