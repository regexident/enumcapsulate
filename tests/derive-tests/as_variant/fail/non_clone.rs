use enumcapsulate::AsVariantRef;

pub struct VariantA;
pub struct VariantB;

#[derive(AsVariant)]
pub enum Enum {
    Unit,
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
}

fn main() {
    let subject = Enum::Unit;

    let _: Option<VariantA> = subject.as_variant();
    let _: Option<VariantB> = subject.as_variant();
}
