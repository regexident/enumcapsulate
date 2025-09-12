use enumcapsulate::VariantDiscriminant;
pub enum VariantWithLifetime<'a> {
    Variant(&'a ()),
}
pub enum VariantWithLifetimeDiscriminant {
    Variant,
}
#[automatically_derived]
impl ::core::marker::Copy for VariantWithLifetimeDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for VariantWithLifetimeDiscriminant {
    #[inline]
    fn clone(&self) -> VariantWithLifetimeDiscriminant {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for VariantWithLifetimeDiscriminant {
    #[inline]
    fn cmp(&self, other: &VariantWithLifetimeDiscriminant) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VariantWithLifetimeDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VariantWithLifetimeDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for VariantWithLifetimeDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for VariantWithLifetimeDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for VariantWithLifetimeDiscriminant {
    #[inline]
    fn eq(&self, other: &VariantWithLifetimeDiscriminant) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::hash::Hash for VariantWithLifetimeDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
}
#[automatically_derived]
impl ::core::fmt::Debug for VariantWithLifetimeDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Variant")
    }
}
impl<'a> ::enumcapsulate::VariantDiscriminant for VariantWithLifetime<'a> {
    type Discriminant = VariantWithLifetimeDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            VariantWithLifetime::Variant(..) => VariantWithLifetimeDiscriminant::Variant,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
pub enum EnumWithLifetime<'a> {
    #[enumcapsulate(discriminant(nested))]
    VariantA(VariantWithLifetime<'a>),
}
pub enum EnumWithLifetimeDiscriminant {
    VariantA(
        <VariantWithLifetime<
            'static,
        > as ::enumcapsulate::VariantDiscriminant>::Discriminant,
    ),
}
#[automatically_derived]
impl ::core::marker::Copy for EnumWithLifetimeDiscriminant {}
#[automatically_derived]
impl ::core::clone::Clone for EnumWithLifetimeDiscriminant {
    #[inline]
    fn clone(&self) -> EnumWithLifetimeDiscriminant {
        let _: ::core::clone::AssertParamIsClone<
            <VariantWithLifetime<
                'static,
            > as ::enumcapsulate::VariantDiscriminant>::Discriminant,
        >;
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for EnumWithLifetimeDiscriminant {
    #[inline]
    fn cmp(&self, other: &EnumWithLifetimeDiscriminant) -> ::core::cmp::Ordering {
        match (self, other) {
            (
                EnumWithLifetimeDiscriminant::VariantA(__self_0),
                EnumWithLifetimeDiscriminant::VariantA(__arg1_0),
            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
        }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for EnumWithLifetimeDiscriminant {
    #[inline]
    fn partial_cmp(
        &self,
        other: &EnumWithLifetimeDiscriminant,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match (self, other) {
            (
                EnumWithLifetimeDiscriminant::VariantA(__self_0),
                EnumWithLifetimeDiscriminant::VariantA(__arg1_0),
            ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumWithLifetimeDiscriminant {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            <VariantWithLifetime<
                'static,
            > as ::enumcapsulate::VariantDiscriminant>::Discriminant,
        >;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumWithLifetimeDiscriminant {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumWithLifetimeDiscriminant {
    #[inline]
    fn eq(&self, other: &EnumWithLifetimeDiscriminant) -> bool {
        match (self, other) {
            (
                EnumWithLifetimeDiscriminant::VariantA(__self_0),
                EnumWithLifetimeDiscriminant::VariantA(__arg1_0),
            ) => __self_0 == __arg1_0,
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EnumWithLifetimeDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match self {
            EnumWithLifetimeDiscriminant::VariantA(__self_0) => {
                ::core::hash::Hash::hash(__self_0, state)
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumWithLifetimeDiscriminant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumWithLifetimeDiscriminant::VariantA(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "VariantA",
                    &__self_0,
                )
            }
        }
    }
}
impl<'a> ::enumcapsulate::VariantDiscriminant for EnumWithLifetime<'a> {
    type Discriminant = EnumWithLifetimeDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            EnumWithLifetime::VariantA(inner, ..) => {
                EnumWithLifetimeDiscriminant::VariantA(inner.variant_discriminant())
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
fn main() {}
