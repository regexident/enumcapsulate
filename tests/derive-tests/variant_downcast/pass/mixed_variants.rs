use enumcapsulate::derive::{AsVariantMut, AsVariantRef, IntoVariant};

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;
#[derive(Clone)]
pub struct VariantC;
#[derive(Clone)]
pub struct VariantD;

#[derive(AsVariantRef, AsVariantMut, IntoVariant)]
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

impl enumcapsulate::VariantDowncast for Enum {}

fn main() {
    use enumcapsulate::VariantDowncast;

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
