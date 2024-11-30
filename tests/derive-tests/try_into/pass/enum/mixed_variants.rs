use enumcapsulate::TryInto;

pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;

#[derive(TryInto)]
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
    {
        let subject = Enum::Unit;
        let _: Result<VariantA, Enum> = subject.try_into();
    }
    {
        let subject = Enum::Unit;
        let _: Result<VariantB, Enum> = subject.try_into();
    }
}
