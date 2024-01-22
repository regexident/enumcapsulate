use enumcapsulate::{derive::Encapsulate, AsVariantMut, AsVariantRef, IntoVariant};
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::core::convert::From<VariantA> for Enum {
    fn from(inner: VariantA) -> Self {
        Self::VariantA(inner)
    }
}
impl ::core::convert::From<VariantB> for Enum {
    fn from(inner: VariantB) -> Self {
        Self::VariantB(inner)
    }
}
impl ::core::convert::TryFrom<Enum> for VariantA {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::VariantA(inner) => Ok(inner),
            err => Err(err),
        }
    }
}
impl ::core::convert::TryFrom<Enum> for VariantB {
    type Error = Enum;
    fn try_from(outer: Enum) -> Result<Self, Self::Error> {
        match outer {
            Enum::VariantB(inner) => Ok(inner),
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
        Self::VariantB(inner)
    }
}
impl ::enumcapsulate::AsVariantRef<VariantA> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::VariantA(variant) => Some(variant),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantRef<VariantB> for Enum {
    fn as_variant_ref(&self) -> Option<&VariantB> {
        match self {
            Enum::VariantB(variant) => Some(variant),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantA> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::VariantA(variant) => Some(variant),
            _ => None,
        }
    }
}
impl ::enumcapsulate::AsVariantMut<VariantB> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut VariantB> {
        match self {
            Enum::VariantB(variant) => Some(variant),
            _ => None,
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantA> for Enum {
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::VariantA(variant) => Ok(variant),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::IntoVariant<VariantB> for Enum {
    fn into_variant(self) -> Result<VariantB, Self> {
        match self {
            Enum::VariantB(variant) => Ok(variant),
            err => Err(err),
        }
    }
}
impl ::enumcapsulate::VariantDowncast for Enum {}
impl ::enumcapsulate::IsVariant for Enum {
    fn is_variant<T>(&self) -> bool
    where
        T: 'static + ?Sized,
    {
        use ::std::any::TypeId;
        #[inline]
        pub fn type_id_of_val<T: 'static + ?Sized>(_val: &T) -> TypeId {
            TypeId::of::<T>()
        }
        let type_id = TypeId::of::<T>();
        match self {
            Enum::VariantA(variant) => type_id_of_val(variant) == type_id,
            Enum::VariantB(variant) => type_id_of_val(variant) == type_id,
        }
    }
}
fn check<T, U>()
where
    T: AsVariantRef<U> + AsVariantMut<U> + IntoVariant<U> + From<U> + TryInto<U>,
{}
fn main() {
    check::<Enum, VariantA>();
    check::<Enum, VariantB>();
}
