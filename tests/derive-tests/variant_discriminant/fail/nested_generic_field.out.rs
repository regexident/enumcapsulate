use enumcapsulate::VariantDiscriminant;
pub enum VariantA<T> {
    Variant(T),
}
pub enum VariantADiscriminant {
    Variant,
}
#[automatically_derived]
impl ::core::marker::Copy for VariantADiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for VariantADiscriminant {
    #[inline]
    fn clone(&self) -> VariantADiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for VariantADiscriminant {
    #[inline]
    fn cmp(&self, other: &VariantADiscriminant) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VariantADiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VariantADiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for VariantADiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for VariantADiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for VariantADiscriminant {
    #[inline]
    fn eq(&self, other: &VariantADiscriminant) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::hash::Hash for VariantADiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
}
#[automatically_derived]
impl ::core::fmt::Debug for VariantADiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Variant")
    }
}
impl<T> ::enumcapsulate::VariantDiscriminant for VariantA<T> {
    type Discriminant = VariantADiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            VariantA::Variant(..) => VariantADiscriminant::Variant,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum<T> {
    #[enumcapsulate(discriminant(nested))]
    VariantA(VariantA<T>),
}
fn main() {}
