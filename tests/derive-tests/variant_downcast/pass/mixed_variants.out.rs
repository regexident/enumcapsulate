use enumcapsulate::derive::{AsVariantMut, AsVariantRef, IntoVariant};
pub struct VariantA;
#[automatically_derived]
impl ::core::clone::Clone for VariantA {
    #[inline]
    fn clone(&self) -> VariantA {
        VariantA
    }
}
pub struct VariantB;
#[automatically_derived]
impl ::core::clone::Clone for VariantB {
    #[inline]
    fn clone(&self) -> VariantB {
        VariantB
    }
}
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(bool),
}
impl ::enumcapsulate::AsVariantRef<VariantA> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::OneTupleField(inner) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantB> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantB> {
        match self {
            Enum::OneStructField { variant: inner } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::OneTupleField(inner) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::OneStructField { variant: inner } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::OneTupleField(inner) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::OneStructField { variant: inner } => Ok(inner),
            err => Err(err),
        }
    }
}
impl enumcapsulate::VariantDowncast for Enum {}
fn main() {
    use enumcapsulate::VariantDowncast;
    let mut subject = Enum::Unit;
    {
        let _: Option<&VariantA> = subject.as_variant_downcast_ref();
        let _: Option<&VariantB> = subject.as_variant_downcast_ref();
    }
    {
        let _: Option<&mut VariantA> = subject.as_variant_downcast_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_downcast_mut();
    }
    {
        let _: Option<VariantA> = subject.as_variant_downcast();
        let _: Option<VariantB> = subject.as_variant_downcast();
    }
    {
        let _: Result<VariantA, Enum> = subject.into_variant_downcast();
    }
}
