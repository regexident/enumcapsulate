use enumcapsulate::VariantDiscriminant;
#[enumcapsulate(discriminant(name = RenamedDiscriminant))]
pub enum Enum {
    VariantA,
    VariantB,
    VariantC,
    #[enumcapsulate(discriminant(name = RenamedVariant))]
    VariantD,
}
pub enum RenamedDiscriminant {
    VariantA,
    VariantB,
    VariantC,
    RenamedVariant,
}
#[automatically_derived]
impl ::core::marker::Copy for RenamedDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for RenamedDiscriminant {
    #[inline]
    fn clone(&self) -> RenamedDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for RenamedDiscriminant {
    #[inline]
    fn cmp(&self, other: &RenamedDiscriminant) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for RenamedDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &RenamedDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RenamedDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RenamedDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RenamedDiscriminant {
    #[inline]
    fn eq(&self, other: &RenamedDiscriminant) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::hash::Hash for RenamedDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RenamedDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                RenamedDiscriminant::VariantA => "VariantA",
                RenamedDiscriminant::VariantB => "VariantB",
                RenamedDiscriminant::VariantC => "VariantC",
                RenamedDiscriminant::RenamedVariant => "RenamedVariant",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = RenamedDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::VariantA => RenamedDiscriminant::VariantA,
            Enum::VariantB => RenamedDiscriminant::VariantB,
            Enum::VariantC => RenamedDiscriminant::VariantC,
            Enum::VariantD => RenamedDiscriminant::RenamedVariant,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
