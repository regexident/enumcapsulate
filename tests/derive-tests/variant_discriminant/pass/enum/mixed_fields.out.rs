use enumcapsulate::derive::VariantDiscriminant;
use enumcapsulate::VariantDiscriminant;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant_a: VariantA },
    TwoTupleFields(VariantA, VariantB),
    TwoStructFields { variant_a: VariantA, variant_b: VariantB },
}
pub enum EnumDiscriminant {
    Unit,
    ZeroTupleFields,
    ZeroStructFields,
    OneTupleField,
    OneStructField,
    TwoTupleFields,
    TwoStructFields,
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
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EnumDiscriminant {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumDiscriminant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumDiscriminant::Unit => "Unit",
                EnumDiscriminant::ZeroTupleFields => "ZeroTupleFields",
                EnumDiscriminant::ZeroStructFields => "ZeroStructFields",
                EnumDiscriminant::OneTupleField => "OneTupleField",
                EnumDiscriminant::OneStructField => "OneStructField",
                EnumDiscriminant::TwoTupleFields => "TwoTupleFields",
                EnumDiscriminant::TwoStructFields => "TwoStructFields",
            },
        )
    }
}
impl ::enumcapsulate::VariantDiscriminant for Enum {
    type Discriminant = EnumDiscriminant;
    fn variant_discriminant(&self) -> Self::Discriminant {
        match self {
            Enum::Unit => EnumDiscriminant::Unit,
            Enum::ZeroTupleFields(..) => EnumDiscriminant::ZeroTupleFields,
            Enum::ZeroStructFields { .. } => EnumDiscriminant::ZeroStructFields,
            Enum::OneTupleField(..) => EnumDiscriminant::OneTupleField,
            Enum::OneStructField { .. } => EnumDiscriminant::OneStructField,
            Enum::TwoTupleFields(..) => EnumDiscriminant::TwoTupleFields,
            Enum::TwoStructFields { .. } => EnumDiscriminant::TwoStructFields,
        }
    }
}
fn check<T>()
where
    T: VariantDiscriminant,
{}
fn main() {
    check::<Enum>();
    check::<Enum>();
}
