use enumcapsulate::IsVariant;

pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;

#[derive(IsVariant)]
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
    #[enumcapsulate(field(index = 1))]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(field(name = "variant"))]
    IncludedStruct {
        value: u8,
        variant: VariantD,
    },
}

fn check<T>()
where
    T: IsVariant,
{
}

fn main() {
    check::<Enum>();
}
