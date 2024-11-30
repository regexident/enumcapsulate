use enumcapsulate::VariantDiscriminant;
pub enum PubEnum {}
pub enum PubEnumDiscriminant {}
#[automatically_derived]
impl ::core::marker::Copy for PubEnumDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for PubEnumDiscriminant {
    #[inline]
    fn clone(&self) -> PubEnumDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for PubEnumDiscriminant {
    #[inline]
    fn cmp(&self, other: &PubEnumDiscriminant) -> ::core::cmp::Ordering {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for PubEnumDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &PubEnumDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for PubEnumDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for PubEnumDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for PubEnumDiscriminant {
    #[inline]
    fn eq(&self, other: &PubEnumDiscriminant) -> bool {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::hash::Hash for PubEnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for PubEnumDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
impl ::enumcapsulate::VariantDiscriminant for PubEnum {
    type Discriminant = PubEnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
pub(crate) enum PubCrateEnum {}
pub(crate) enum PubCrateEnumDiscriminant {}
#[automatically_derived]
impl ::core::marker::Copy for PubCrateEnumDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for PubCrateEnumDiscriminant {
    #[inline]
    fn clone(&self) -> PubCrateEnumDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for PubCrateEnumDiscriminant {
    #[inline]
    fn cmp(&self, other: &PubCrateEnumDiscriminant) -> ::core::cmp::Ordering {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for PubCrateEnumDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &PubCrateEnumDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for PubCrateEnumDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for PubCrateEnumDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for PubCrateEnumDiscriminant {
    #[inline]
    fn eq(&self, other: &PubCrateEnumDiscriminant) -> bool {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::hash::Hash for PubCrateEnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for PubCrateEnumDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
impl ::enumcapsulate::VariantDiscriminant for PubCrateEnum {
    type Discriminant = PubCrateEnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
pub(self) enum PubSelfEnum {}
pub(self) enum PubSelfEnumDiscriminant {}
#[automatically_derived]
impl ::core::marker::Copy for PubSelfEnumDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for PubSelfEnumDiscriminant {
    #[inline]
    fn clone(&self) -> PubSelfEnumDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for PubSelfEnumDiscriminant {
    #[inline]
    fn cmp(&self, other: &PubSelfEnumDiscriminant) -> ::core::cmp::Ordering {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for PubSelfEnumDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &PubSelfEnumDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for PubSelfEnumDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for PubSelfEnumDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for PubSelfEnumDiscriminant {
    #[inline]
    fn eq(&self, other: &PubSelfEnumDiscriminant) -> bool {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::hash::Hash for PubSelfEnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for PubSelfEnumDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
impl ::enumcapsulate::VariantDiscriminant for PubSelfEnum {
    type Discriminant = PubSelfEnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
