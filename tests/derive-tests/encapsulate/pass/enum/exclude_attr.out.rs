use enumcapsulate::Encapsulate;
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
#[enumcapsulate(exclude(From, TryInto))]
pub enum Enum {
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    #[enumcapsulate(exclude)]
    ExcludedWildcard(VariantC),
    #[enumcapsulate(exclude(FromVariant, AsVariant))]
    ExcludedSelective(VariantD),
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
impl ::enumcapsulate::AsVariantRef<VariantD> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantD> {
        match self {
            Enum::ExcludedSelective(inner, ..) => Some(inner),
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
impl ::enumcapsulate::AsVariantMut<VariantD> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantD> {
        match self {
            Enum::ExcludedSelective(inner, ..) => Some(inner),
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
impl ::enumcapsulate::IntoVariant<VariantD> for Enum {
    fn into_variant(self) -> Result<VariantD, Self> {
        match self {
            Enum::ExcludedSelective(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::VariantDowncast for Enum {}
pub enum EnumDiscriminant {
    OneTupleField,
    OneStructField,
    ExcludedWildcard,
    ExcludedSelective,
}
#[automatically_derived]
impl ::core::marker::Copy for EnumDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for EnumDiscriminant {
    #[inline]
    fn clone(&self) -> EnumDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumDiscriminant {
    #[inline]
    fn eq(&self, other: &EnumDiscriminant) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumDiscriminant::OneTupleField => "OneTupleField",
                EnumDiscriminant::OneStructField => "OneStructField",
                EnumDiscriminant::ExcludedWildcard => "ExcludedWildcard",
                EnumDiscriminant::ExcludedSelective => "ExcludedSelective",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = EnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::OneTupleField(..) => EnumDiscriminant::OneTupleField,
            Enum::OneStructField { .. } => EnumDiscriminant::OneStructField,
            Enum::ExcludedWildcard(..) => EnumDiscriminant::ExcludedWildcard,
            Enum::ExcludedSelective(..) => EnumDiscriminant::ExcludedSelective,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
