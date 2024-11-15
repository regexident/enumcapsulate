use enumcapsulate::{
    AsVariantMut, AsVariantRef, Encapsulate, IntoVariant, VariantDowncast,
};
pub enum Enum {}
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
            _ => false,
        }
    }
}
pub enum EnumDiscriminant {}
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
        match *self {}
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumDiscriminant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = EnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn check<T>()
where
    T: VariantDowncast,
{}
fn main() {
    check::<Enum>();
}
