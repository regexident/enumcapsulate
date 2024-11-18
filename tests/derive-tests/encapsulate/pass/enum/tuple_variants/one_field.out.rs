use enumcapsulate::{
    AsVariant, AsVariantMut, AsVariantRef, Encapsulate, FromVariant, IntoVariant,
};
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
    VariantA(VariantA),
    VariantB { b: VariantB },
}
impl ::core::convert::From<VariantA> for Enum {
    fn from(inner: VariantA) -> Self {
        Self::VariantA(inner)
    }
}
impl ::core::convert::From<VariantB> for Enum {
    fn from(inner: VariantB) -> Self {
        Self::VariantB { b: inner }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantA {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::VariantA(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantB {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::VariantB { b: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::FromVariant<VariantA> for Enum {
    fn from_variant(inner: VariantA) -> Self {
        Self::VariantA(inner)
    }
}
impl ::enumcapsulate::FromVariant<VariantB> for Enum {
    fn from_variant(inner: VariantB) -> Self {
        Self::VariantB { b: inner }
    }
}
impl ::enumcapsulate::AsVariant<VariantA> for Enum
where
    VariantA: Clone,
{
    fn as_variant(&self) -> Option<VariantA> {
        match self {
            Enum::VariantA(inner, ..) => Some(inner.clone()),
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
            Enum::VariantB { b: inner, .. } => Some(inner.clone()),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantA> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::VariantA(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantB> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantB> {
        match self {
            Enum::VariantB { b: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::VariantA(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::VariantB { b: inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::VariantA(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::VariantB { b: inner, .. } => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::VariantDowncast for Enum {}
pub enum EnumDiscriminant {
    VariantA,
    VariantB,
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
impl ::core::marker::StructuralEq for EnumDiscriminant {}
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
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumDiscriminant::VariantA => "VariantA",
                EnumDiscriminant::VariantB => "VariantB",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = EnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::VariantA(..) => EnumDiscriminant::VariantA,
            Enum::VariantB { .. } => EnumDiscriminant::VariantB,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn check<T, U>()
where
    T: FromVariant<U> + IntoVariant<U> + From<U> + TryInto<U> + AsVariant<U>
        + AsVariantRef<U> + AsVariantMut<U>,
{}
fn main() {
    check::<Enum, VariantA>();
    check::<Enum, VariantB>();
}
