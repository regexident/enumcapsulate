use enumcapsulate::derive::AsVariantRef;

pub struct VariantA;
pub struct VariantB;

#[derive(AsVariantRef)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn main() {
    use enumcapsulate::AsVariantRef;

    let subject = Enum::VariantA(VariantA);

    let _: Option<&VariantA> = subject.as_variant_ref();
    let _: Option<&VariantB> = subject.as_variant_ref();
}
