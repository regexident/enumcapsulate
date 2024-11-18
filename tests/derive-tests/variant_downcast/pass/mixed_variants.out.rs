use enumcapsulate::{AsVariant, AsVariantMut, AsVariantRef, IntoVariant, VariantDowncast};
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
pub struct VariantC;
#[automatically_derived]
impl ::core::clone::Clone for VariantC {
    #[inline]
    fn clone(&self) -> VariantC {
        VariantC
    }
}
pub struct VariantD;
#[automatically_derived]
impl ::core::clone::Clone for VariantD {
    #[inline]
    fn clone(&self) -> VariantD {
        VariantD
    }
}
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    #[enumcapsulate(exclude)]
    TwoTupleFields(i32, u32),
    #[enumcapsulate(exclude)]
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
    #[enumcapsulate(field(index = 1))]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(field(name = "variant"))]
    IncludedStruct { value: u8, variant: VariantD },
}
impl ::enumcapsulate::AsVariant<VariantA> for Enum
where
    VariantA: Clone,
{
    fn as_variant(&self) -> Option<VariantA> {
        match self {
            Enum::OneTupleField(inner, ..) => Some(inner.clone()),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariant<VariantB> for Enum
where
    VariantB: Clone,
{
    fn as_variant(&self) -> Option<VariantB> {
        match self {
            Enum::OneStructField { variant: inner, .. } => Some(inner.clone()),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariant<VariantC> for Enum
where
    VariantC: Clone,
{
    fn as_variant(&self) -> Option<VariantC> {
        match self {
            Enum::IncludedTuple(_, inner, ..) => Some(inner.clone()),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariant<VariantD> for Enum
where
    VariantD: Clone,
{
    fn as_variant(&self) -> Option<VariantD> {
        match self {
            Enum::IncludedStruct { variant: inner, .. } => Some(inner.clone()),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantA> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::OneTupleField(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantB> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantB> {
        match self {
            Enum::OneStructField { variant: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantC> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantC> {
        match self {
            Enum::IncludedTuple(_, inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantD> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantD> {
        match self {
            Enum::IncludedStruct { variant: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::OneTupleField(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::OneStructField { variant: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantC> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantC> {
        match self {
            Enum::IncludedTuple(_, inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantD> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantD> {
        match self {
            Enum::IncludedStruct { variant: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::OneTupleField(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::OneStructField { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantC> for Enum {
    fn into_variant(self) -> Result<VariantC, Self> {
        match self {
            Enum::IncludedTuple(_, inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantD> for Enum {
    fn into_variant(self) -> Result<VariantD, Self> {
        match self {
            Enum::IncludedStruct { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
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
