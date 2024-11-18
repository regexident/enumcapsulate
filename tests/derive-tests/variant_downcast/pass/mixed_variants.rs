use enumcapsulate::{AsVariant, AsVariantMut, AsVariantRef, IntoVariant, VariantDowncast};

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;
#[derive(Clone)]
pub struct VariantC;
#[derive(Clone)]
pub struct VariantD;

#[derive(AsVariant, AsVariantRef, AsVariantMut, IntoVariant)]
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

impl VariantDowncast for Enum {}

fn main() {
    let mut subject = Enum::Unit;

    {
        let _: Option<&VariantA> = subject.as_variant_downcast_ref();
        let _: Option<&VariantB> = subject.as_variant_downcast_ref();
        let _: Option<&VariantC> = subject.as_variant_downcast_ref();
        let _: Option<&VariantD> = subject.as_variant_downcast_ref();
    }

    {
        let _: Option<&mut VariantA> = subject.as_variant_downcast_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_downcast_mut();
    }
    {
        let _: Option<&mut VariantC> = subject.as_variant_downcast_mut();
    }
    {
        let _: Option<&mut VariantD> = subject.as_variant_downcast_mut();
    }

    {
        let _: Option<VariantA> = subject.as_variant_downcast();
        let _: Option<VariantB> = subject.as_variant_downcast();
        let _: Option<VariantC> = subject.as_variant_downcast();
        let _: Option<VariantD> = subject.as_variant_downcast();
    }

    {
        let mut subject = Enum::Unit;
        let _: Result<VariantA, Enum> = subject.into_variant_downcast();
    }
    {
        let mut subject = Enum::Unit;
        let _: Result<VariantB, Enum> = subject.into_variant_downcast();
    }
    {
        let mut subject = Enum::Unit;
        let _: Result<VariantC, Enum> = subject.into_variant_downcast();
    }
    {
        let mut subject = Enum::Unit;
        let _: Result<VariantD, Enum> = subject.into_variant_downcast();
    }
}
