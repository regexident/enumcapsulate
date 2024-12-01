use enumcapsulate::{Encapsulate, VariantDiscriminant};
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
#[enumcapsulate(discriminant(name = CustomDiscriminant, repr = u8))]
pub enum Enum {
    #[enumcapsulate(discriminant(name = CustomVariant, value = 42))]
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    #[enumcapsulate(exclude(From, TryInto))]
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    #[enumcapsulate(exclude)]
    TwoTupleFields(i32, u32),
    #[enumcapsulate(exclude)]
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(VariantA, VariantB),
    #[enumcapsulate(field = 1)]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(field = "variant")]
    IncludedStruct { value: u8, variant: VariantD },
}
impl ::core::convert::From<VariantB> for Enum {
    fn from(inner: VariantB) -> Self {
        Self::OneStructField {
            variant: inner,
        }
    }
}
impl ::core::convert::From<VariantC> for Enum {
    fn from(inner: VariantC) -> Self {
        Self::IncludedTuple(Default::default(), inner)
    }
}
impl ::core::convert::From<VariantD> for Enum {
    fn from(inner: VariantD) -> Self {
        Self::IncludedStruct {
            value: Default::default(),
            variant: inner,
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantB {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::OneStructField { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantC {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::IncludedTuple(_, inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantD {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::IncludedStruct { variant: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::FromVariant<VariantA> for Enum {
    fn from_variant(inner: VariantA) -> Self {
        Self::OneTupleField(inner)
    }
}
impl ::enumcapsulate::FromVariant<VariantB> for Enum {
    fn from_variant(inner: VariantB) -> Self {
        Self::OneStructField {
            variant: inner,
        }
    }
}
impl ::enumcapsulate::FromVariant<VariantC> for Enum {
    fn from_variant(inner: VariantC) -> Self {
        Self::IncludedTuple(Default::default(), inner)
    }
}
impl ::enumcapsulate::FromVariant<VariantD> for Enum {
    fn from_variant(inner: VariantD) -> Self {
        Self::IncludedStruct {
            value: Default::default(),
            variant: inner,
        }
    }
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
impl ::enumcapsulate::VariantDowncast for Enum {}
#[repr(u8)]
pub enum CustomDiscriminant {
    CustomVariant = 42,
    ZeroTupleFields,
    ZeroStructFields,
    OneTupleField,
    OneStructField,
    TwoTupleFields,
    TwoStructFields,
    Excluded,
    IncludedTuple,
    IncludedStruct,
}
#[automatically_derived]
impl ::core::marker::Copy for CustomDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for CustomDiscriminant {
    #[inline]
    fn clone(&self) -> CustomDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for CustomDiscriminant {
    #[inline]
    fn cmp(&self, other: &CustomDiscriminant) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for CustomDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &CustomDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for CustomDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for CustomDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for CustomDiscriminant {
    #[inline]
    fn eq(&self, other: &CustomDiscriminant) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for CustomDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CustomDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                CustomDiscriminant::CustomVariant => "CustomVariant",
                CustomDiscriminant::ZeroTupleFields => "ZeroTupleFields",
                CustomDiscriminant::ZeroStructFields => "ZeroStructFields",
                CustomDiscriminant::OneTupleField => "OneTupleField",
                CustomDiscriminant::OneStructField => "OneStructField",
                CustomDiscriminant::TwoTupleFields => "TwoTupleFields",
                CustomDiscriminant::TwoStructFields => "TwoStructFields",
                CustomDiscriminant::Excluded => "Excluded",
                CustomDiscriminant::IncludedTuple => "IncludedTuple",
                CustomDiscriminant::IncludedStruct => "IncludedStruct",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = CustomDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::Unit => CustomDiscriminant::CustomVariant,
            Enum::ZeroTupleFields(..) => CustomDiscriminant::ZeroTupleFields,
            Enum::ZeroStructFields { .. } => CustomDiscriminant::ZeroStructFields,
            Enum::OneTupleField(..) => CustomDiscriminant::OneTupleField,
            Enum::OneStructField { .. } => CustomDiscriminant::OneStructField,
            Enum::TwoTupleFields(..) => CustomDiscriminant::TwoTupleFields,
            Enum::TwoStructFields { .. } => CustomDiscriminant::TwoStructFields,
            Enum::Excluded(..) => CustomDiscriminant::Excluded,
            Enum::IncludedTuple(..) => CustomDiscriminant::IncludedTuple,
            Enum::IncludedStruct { .. } => CustomDiscriminant::IncludedStruct,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
