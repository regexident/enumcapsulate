use enumcapsulate::VariantDiscriminant;
pub enum Enum {}
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
impl ::core::cmp::Ord for EnumDiscriminant {
    #[inline]
    fn cmp(&self, other: &EnumDiscriminant) -> ::core::cmp::Ordering {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for EnumDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &EnumDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match *self {}
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
    #[inline]
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
fn main() {}
