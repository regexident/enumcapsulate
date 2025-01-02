use enumcapsulate::VariantDiscriminant;
pub enum VariantA {
    VariantA1,
    VariantA2,
}
pub enum VariantADiscriminant {
    VariantA1,
    VariantA2,
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
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VariantADiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VariantADiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
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
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for VariantADiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for VariantADiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                VariantADiscriminant::VariantA1 => "VariantA1",
                VariantADiscriminant::VariantA2 => "VariantA2",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for VariantA {
    type Discriminant = VariantADiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            VariantA::VariantA1 => VariantADiscriminant::VariantA1,
            VariantA::VariantA2 => VariantADiscriminant::VariantA2,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
pub enum Enum {
    #[enumcapsulate(discriminant(nested, value = 42))]
    VariantA(VariantA),
}
fn main() {}
