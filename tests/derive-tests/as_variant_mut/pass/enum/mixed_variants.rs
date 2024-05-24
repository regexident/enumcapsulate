use enumcapsulate::derive::AsVariantMut;

pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;

#[derive(AsVariantMut)]
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
    #[enumcapsulate(include(field = 1))]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(include(field = "variant"))]
    IncludedStruct {
        value: u8,
        variant: VariantD,
    },
}

fn main() {
    use enumcapsulate::AsVariantMut;

    let mut subject = Enum::Unit;

    {
        let _: Option<&mut VariantA> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_mut();
    }
}
