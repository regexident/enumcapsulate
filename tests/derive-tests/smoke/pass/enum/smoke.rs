use enumcapsulate::{Encapsulate, VariantDiscriminant};

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;
#[derive(Clone)]
pub struct VariantC;
#[derive(Clone)]
pub struct VariantD;

#[derive(Encapsulate, VariantDiscriminant)]
#[enumcapsulate(discriminant(name = CustomDiscriminant, repr = u8))]
pub enum Enum {
    #[enumcapsulate(discriminant(name = CustomVariant, value = 42))]
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    #[enumcapsulate(exclude(From, TryInto))]
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

fn main() {}
