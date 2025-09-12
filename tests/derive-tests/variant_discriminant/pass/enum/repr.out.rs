use enumcapsulate::VariantDiscriminant;
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum {
    VariantA,
    VariantB = 5,
    VariantC,
    #[enumcapsulate(discriminant(value = 42))]
    VariantD,
}
#[repr(u8)]
pub enum EnumDiscriminant {
    VariantA,
    VariantB = 5,
    VariantC,
    VariantD = 42,
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
impl ::core::cmp::Ord for EnumDiscriminant {
    #[inline]
    fn cmp(&self, other: &EnumDiscriminant) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for EnumDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &EnumDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
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
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumDiscriminant::VariantA => "VariantA",
                EnumDiscriminant::VariantB => "VariantB",
                EnumDiscriminant::VariantC => "VariantC",
                EnumDiscriminant::VariantD => "VariantD",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = EnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::VariantA => EnumDiscriminant::VariantA,
            Enum::VariantB => EnumDiscriminant::VariantB,
            Enum::VariantC => EnumDiscriminant::VariantC,
            Enum::VariantD => EnumDiscriminant::VariantD,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
