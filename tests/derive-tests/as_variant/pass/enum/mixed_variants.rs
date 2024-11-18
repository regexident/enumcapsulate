use enumcapsulate::AsVariant;

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;
pub struct VariantC;

#[derive(AsVariant)]
pub enum Enum {
    Unit,
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
}

fn main() {
    let subject = Enum::Unit;

    let _: Option<VariantA> = subject.as_variant();
    let _: Option<VariantB> = subject.as_variant();
}
