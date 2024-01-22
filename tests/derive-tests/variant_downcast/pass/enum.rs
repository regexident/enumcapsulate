use enumcapsulate::derive::{AsVariantMut, AsVariantRef, IntoVariant};

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;

#[derive(AsVariantRef, AsVariantMut, IntoVariant)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

impl enumcapsulate::VariantDowncast for Enum {}

fn main() {
    use enumcapsulate::VariantDowncast;

    let mut subject = Enum::VariantA(VariantA);

    {
        let _: Option<&VariantA> = subject.as_variant_downcast_ref();
    }
    {
        let _: Option<&mut VariantA> = subject.as_variant_downcast_mut();
    }
    {
        let _: Option<VariantA> = subject.as_variant_downcast();
    }
    {
        let _: Result<VariantA, Enum> = subject.into_variant_downcast();
    }
}
