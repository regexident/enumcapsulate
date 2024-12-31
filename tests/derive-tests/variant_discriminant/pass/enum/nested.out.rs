use enumcapsulate::VariantDiscriminant;
pub enum VariantC {
    VariantC1,
    VariantC2,
}
pub enum VariantCDiscriminant {
    VariantC1,
    VariantC2,
}
#[automatically_derived]
impl ::core::marker::Copy for VariantCDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for VariantCDiscriminant {
    #[inline]
    fn clone(&self) -> VariantCDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for VariantCDiscriminant {
    #[inline]
    fn cmp(&self, other: &VariantCDiscriminant) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VariantCDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VariantCDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for VariantCDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for VariantCDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for VariantCDiscriminant {
    #[inline]
    fn eq(&self, other: &VariantCDiscriminant) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for VariantCDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for VariantCDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                VariantCDiscriminant::VariantC1 => "VariantC1",
                VariantCDiscriminant::VariantC2 => "VariantC2",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for VariantC {
    type Discriminant = VariantCDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            VariantC::VariantC1 => VariantCDiscriminant::VariantC1,
            VariantC::VariantC2 => VariantCDiscriminant::VariantC2,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
pub enum VariantD {
    VariantD1,
    VariantD2,
}
pub enum VariantDDiscriminant {
    VariantD1,
    VariantD2,
}
#[automatically_derived]
impl ::core::marker::Copy for VariantDDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for VariantDDiscriminant {
    #[inline]
    fn clone(&self) -> VariantDDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for VariantDDiscriminant {
    #[inline]
    fn cmp(&self, other: &VariantDDiscriminant) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VariantDDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VariantDDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for VariantDDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for VariantDDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for VariantDDiscriminant {
    #[inline]
    fn eq(&self, other: &VariantDDiscriminant) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for VariantDDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for VariantDDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                VariantDDiscriminant::VariantD1 => "VariantD1",
                VariantDDiscriminant::VariantD2 => "VariantD2",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for VariantD {
    type Discriminant = VariantDDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            VariantD::VariantD1 => VariantDDiscriminant::VariantD1,
            VariantD::VariantD2 => VariantDDiscriminant::VariantD2,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
pub enum VariantE {
    VariantE1,
    VariantE2,
}
pub enum VariantEDiscriminant {
    VariantE1,
    VariantE2,
}
#[automatically_derived]
impl ::core::marker::Copy for VariantEDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for VariantEDiscriminant {
    #[inline]
    fn clone(&self) -> VariantEDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for VariantEDiscriminant {
    #[inline]
    fn cmp(&self, other: &VariantEDiscriminant) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VariantEDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VariantEDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for VariantEDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for VariantEDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for VariantEDiscriminant {
    #[inline]
    fn eq(&self, other: &VariantEDiscriminant) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for VariantEDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for VariantEDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                VariantEDiscriminant::VariantE1 => "VariantE1",
                VariantEDiscriminant::VariantE2 => "VariantE2",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for VariantE {
    type Discriminant = VariantEDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            VariantE::VariantE1 => VariantEDiscriminant::VariantE1,
            VariantE::VariantE2 => VariantEDiscriminant::VariantE2,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[enumcapsulate(discriminant(repr = u8))]
pub enum Enum {
    VariantA,
    #[enumcapsulate(discriminant(value = 42))]
    VariantB,
    #[enumcapsulate(discriminant(nested = VariantCDiscriminant))]
    VariantC { c: VariantC },
    #[enumcapsulate(discriminant(name = RenamedVariant, nested = VariantDDiscriminant))]
    VariantD(VariantD),
    #[enumcapsulate(field = 1, discriminant(nested = VariantEDiscriminant))]
    VariantE(bool, VariantE),
}
#[repr(u8)]
pub enum EnumDiscriminant {
    VariantA,
    VariantB = 42,
    VariantC(VariantCDiscriminant),
    RenamedVariant(VariantDDiscriminant),
    VariantE(VariantEDiscriminant),
}
#[automatically_derived]
impl ::core::marker::Copy for EnumDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for EnumDiscriminant {
    #[inline]
    fn clone(&self) -> EnumDiscriminant {
        let _: ::core::clone::AssertParamIsClone<VariantCDiscriminant>;
        let _: ::core::clone::AssertParamIsClone<VariantDDiscriminant>;
        let _: ::core::clone::AssertParamIsClone<VariantEDiscriminant>;
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for EnumDiscriminant {
    #[inline]
    fn cmp(&self, other: &EnumDiscriminant) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal => {
                match (self, other) {
                    (
                        EnumDiscriminant::VariantC(__self_0),
                        EnumDiscriminant::VariantC(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (
                        EnumDiscriminant::RenamedVariant(__self_0),
                        EnumDiscriminant::RenamedVariant(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (
                        EnumDiscriminant::VariantE(__self_0),
                        EnumDiscriminant::VariantE(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::Ordering::Equal,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for EnumDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &EnumDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (
                EnumDiscriminant::VariantC(__self_0),
                EnumDiscriminant::VariantC(__arg1_0),
            ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (
                EnumDiscriminant::RenamedVariant(__self_0),
                EnumDiscriminant::RenamedVariant(__arg1_0),
            ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (
                EnumDiscriminant::VariantE(__self_0),
                EnumDiscriminant::VariantE(__arg1_0),
            ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<VariantCDiscriminant>;
        let _: ::core::cmp::AssertParamIsEq<VariantDDiscriminant>;
        let _: ::core::cmp::AssertParamIsEq<VariantEDiscriminant>;
    }
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
            && match (self, other) {
                (
                    EnumDiscriminant::VariantC(__self_0),
                    EnumDiscriminant::VariantC(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (
                    EnumDiscriminant::RenamedVariant(__self_0),
                    EnumDiscriminant::RenamedVariant(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (
                    EnumDiscriminant::VariantE(__self_0),
                    EnumDiscriminant::VariantE(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            EnumDiscriminant::VariantC(__self_0) => {
                ::core::hash::Hash::hash(__self_0, state)
            }
            EnumDiscriminant::RenamedVariant(__self_0) => {
                ::core::hash::Hash::hash(__self_0, state)
            }
            EnumDiscriminant::VariantE(__self_0) => {
                ::core::hash::Hash::hash(__self_0, state)
            }
            _ => {}
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumDiscriminant::VariantA => {
                ::core::fmt::Formatter::write_str(f, "VariantA")
            }
            EnumDiscriminant::VariantB => {
                ::core::fmt::Formatter::write_str(f, "VariantB")
            }
            EnumDiscriminant::VariantC(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "VariantC",
                    &__self_0,
                )
            }
            EnumDiscriminant::RenamedVariant(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "RenamedVariant",
                    &__self_0,
                )
            }
            EnumDiscriminant::VariantE(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "VariantE",
                    &__self_0,
                )
            }
        }
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = EnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::VariantA => EnumDiscriminant::VariantA,
            Enum::VariantB => EnumDiscriminant::VariantB,
            Enum::VariantC { c: inner, .. } => {
                EnumDiscriminant::VariantC(inner.variant_discriminant())
            }
            Enum::VariantD(inner, ..) => {
                EnumDiscriminant::RenamedVariant(inner.variant_discriminant())
            }
            Enum::VariantE(_, inner, ..) => {
                EnumDiscriminant::VariantE(inner.variant_discriminant())
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
