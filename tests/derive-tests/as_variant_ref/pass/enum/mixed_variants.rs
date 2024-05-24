use enumcapsulate::derive::AsVariantRef;

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;

#[derive(AsVariantRef)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    TwoTupleFields(i32, u32),
    TwoStructFields {
        a: i32,
        b: u32,
    },
    #[enumcapsulate(exclude)]
    Excluded(bool),
}

fn main() {
    use enumcapsulate::AsVariantRef;

    let subject = Enum::Unit;

    let _: Option<&VariantA> = subject.as_variant_ref();
    let _: Option<&VariantB> = subject.as_variant_ref();

    let _: Option<VariantA> = subject.as_variant();
    let _: Option<VariantB> = subject.as_variant();
}
