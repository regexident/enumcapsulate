use enumcapsulate::AsVariantRef;

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;
#[derive(Clone)]
pub struct VariantC;
#[derive(Clone)]
pub struct VariantD;

#[derive(AsVariantRef)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    #[enumcapsulate(exclude)]
    TwoTupleFields(i32, u32),
    #[enumcapsulate(exclude)]
    TwoStructFields {
        a: i32,
        b: u32,
    },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
    #[enumcapsulate(field = 1)]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(field = "variant")]
    IncludedStruct {
        value: u8,
        variant: VariantD,
    },
}

fn main() {
    let subject = Enum::Unit;

    let _: Option<&VariantA> = subject.as_variant_ref();
    let _: Option<&VariantB> = subject.as_variant_ref();
}
