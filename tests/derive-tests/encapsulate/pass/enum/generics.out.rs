use enumcapsulate::Encapsulate;
trait HasAssoc {
    type Assoc: Sized;
}
pub struct VariantA;
#[automatically_derived]
impl ::core::clone::Clone for VariantA {
    #[inline]
    fn clone(&self) -> VariantA {
        VariantA
    }
}
pub struct VariantB;
#[automatically_derived]
impl ::core::clone::Clone for VariantB {
    #[inline]
    fn clone(&self) -> VariantB {
        VariantB
    }
}
pub struct VariantC<'l>(&'l ());
#[automatically_derived]
impl<'l> ::core::clone::Clone for VariantC<'l> {
    #[inline]
    fn clone(&self) -> VariantC<'l> {
        VariantC(::core::clone::Clone::clone(&self.0))
    }
}
pub struct VariantD<const N: usize>([(); N]);
#[automatically_derived]
impl<const N: usize> ::core::clone::Clone for VariantD<N> {
    #[inline]
    fn clone(&self) -> VariantD<N> {
        VariantD(::core::clone::Clone::clone(&self.0))
    }
}
pub struct VariantE<T>(T);
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for VariantE<T> {
    #[inline]
    fn clone(&self) -> VariantE<T> {
        VariantE(::core::clone::Clone::clone(&self.0))
    }
}
pub struct VariantF<'l, const N: usize, T>(&'l [T; N]);
#[automatically_derived]
impl<'l, const N: usize, T: ::core::clone::Clone> ::core::clone::Clone
for VariantF<'l, N, T> {
    #[inline]
    fn clone(&self) -> VariantF<'l, N, T> {
        VariantF(::core::clone::Clone::clone(&self.0))
    }
}
pub struct VariantT;
impl HasAssoc for VariantT {
    type Assoc = ();
}
pub struct VariantU;
trait HasT {
    type T: Sized;
}
impl HasT for VariantU {
    type T = ();
}
pub enum Enum<'l, const N: usize, T>
where
    T: HasAssoc,
{
    Unit,
    OwnedSpecificType(VariantA),
    BorrowedSpecificType(&'l VariantB),
    OwnedWithGenericLifetime(VariantC<'l>),
    OwnedGenericTypeWithConstParam(VariantD<N>),
    OwnedGenericTypeWithTypeParam(VariantE<T>),
    OwnedGenericTypeWithMixedParams(VariantF<'l, N, T>),
    OwnedGenericParam(T),
    BorrowedGenericParam(&'l T),
    OwnedAssocTypeOfGenericParam(T::Assoc),
    OwnedAssocTypeOfGenericParamBehindCast(<T as HasAssoc>::Assoc),
    OwnedAssocTypeOfSpecificTypeBehindCast(<VariantU as HasT>::T),
}
impl<'l, const N: usize, T> ::core::convert::From<VariantA> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from(inner: VariantA) -> Self {
        Self::OwnedSpecificType(inner)
    }
}
impl<'l, const N: usize, T> ::core::convert::From<&'l VariantB> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from(inner: &'l VariantB) -> Self {
        Self::BorrowedSpecificType(inner)
    }
}
impl<'l, const N: usize, T> ::core::convert::From<VariantC<'l>> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from(inner: VariantC<'l>) -> Self {
        Self::OwnedWithGenericLifetime(inner)
    }
}
impl<'l, const N: usize, T> ::core::convert::From<<VariantU as HasT>::T>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from(inner: <VariantU as HasT>::T) -> Self {
        Self::OwnedAssocTypeOfSpecificTypeBehindCast(inner)
    }
}
impl<'l, const N: usize, T> ::core::convert::TryFrom<Enum<'l, N, T>> for VariantA
where
    T: HasAssoc,
{
    type Error = Enum<'l, N, T>;
    fn try_from(outer: Enum<'l, N, T>) -> Result<Self, Self::Error> {
        match outer {
            Enum::OwnedSpecificType(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::core::convert::TryFrom<Enum<'l, N, T>> for &'l VariantB
where
    T: HasAssoc,
{
    type Error = Enum<'l, N, T>;
    fn try_from(outer: Enum<'l, N, T>) -> Result<Self, Self::Error> {
        match outer {
            Enum::BorrowedSpecificType(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::core::convert::TryFrom<Enum<'l, N, T>> for VariantC<'l>
where
    T: HasAssoc,
{
    type Error = Enum<'l, N, T>;
    fn try_from(outer: Enum<'l, N, T>) -> Result<Self, Self::Error> {
        match outer {
            Enum::OwnedWithGenericLifetime(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::core::convert::TryFrom<Enum<'l, N, T>>
for <VariantU as HasT>::T
where
    T: HasAssoc,
{
    type Error = Enum<'l, N, T>;
    fn try_from(outer: Enum<'l, N, T>) -> Result<Self, Self::Error> {
        match outer {
            Enum::OwnedAssocTypeOfSpecificTypeBehindCast(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::FromVariant<VariantA> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from_variant(inner: VariantA) -> Self {
        Self::OwnedSpecificType(inner)
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::FromVariant<&'l VariantB> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from_variant(inner: &'l VariantB) -> Self {
        Self::BorrowedSpecificType(inner)
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::FromVariant<VariantC<'l>> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from_variant(inner: VariantC<'l>) -> Self {
        Self::OwnedWithGenericLifetime(inner)
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::FromVariant<<VariantU as HasT>::T>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn from_variant(inner: <VariantU as HasT>::T) -> Self {
        Self::OwnedAssocTypeOfSpecificTypeBehindCast(inner)
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariant<VariantA> for Enum<'l, N, T>
where
    T: HasAssoc,
    VariantA: Clone,
{
    fn as_variant(&self) -> Option<VariantA> {
        match self {
            Enum::OwnedSpecificType(inner, ..) => Some(inner.clone()),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariant<&'l VariantB> for Enum<'l, N, T>
where
    T: HasAssoc,
    &'l VariantB: Clone,
{
    fn as_variant(&self) -> Option<&'l VariantB> {
        match self {
            Enum::BorrowedSpecificType(inner, ..) => Some(inner.clone()),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariant<VariantC<'l>> for Enum<'l, N, T>
where
    T: HasAssoc,
    VariantC<'l>: Clone,
{
    fn as_variant(&self) -> Option<VariantC<'l>> {
        match self {
            Enum::OwnedWithGenericLifetime(inner, ..) => Some(inner.clone()),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariant<<VariantU as HasT>::T>
for Enum<'l, N, T>
where
    T: HasAssoc,
    <VariantU as HasT>::T: Clone,
{
    fn as_variant(&self) -> Option<<VariantU as HasT>::T> {
        match self {
            Enum::OwnedAssocTypeOfSpecificTypeBehindCast(inner, ..) => {
                Some(inner.clone())
            }
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantRef<VariantA> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_ref(&self) -> Option<&VariantA> {
        match self {
            Enum::OwnedSpecificType(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantRef<&'l VariantB>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_ref(&self) -> Option<&&'l VariantB> {
        match self {
            Enum::BorrowedSpecificType(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantRef<VariantC<'l>>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_ref(&self) -> Option<&VariantC<'l>> {
        match self {
            Enum::OwnedWithGenericLifetime(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantRef<<VariantU as HasT>::T>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_ref(&self) -> Option<&<VariantU as HasT>::T> {
        match self {
            Enum::OwnedAssocTypeOfSpecificTypeBehindCast(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantMut<VariantA> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_mut(&mut self) -> Option<&mut VariantA> {
        match self {
            Enum::OwnedSpecificType(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantMut<&'l VariantB>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_mut(&mut self) -> Option<&mut &'l VariantB> {
        match self {
            Enum::BorrowedSpecificType(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantMut<VariantC<'l>>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_mut(&mut self) -> Option<&mut VariantC<'l>> {
        match self {
            Enum::OwnedWithGenericLifetime(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::AsVariantMut<<VariantU as HasT>::T>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn as_variant_mut(&mut self) -> Option<&mut <VariantU as HasT>::T> {
        match self {
            Enum::OwnedAssocTypeOfSpecificTypeBehindCast(inner, ..) => Some(inner),
            _ => None,
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::IntoVariant<VariantA> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn into_variant(self) -> Result<VariantA, Self> {
        match self {
            Enum::OwnedSpecificType(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::IntoVariant<&'l VariantB> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn into_variant(self) -> Result<&'l VariantB, Self> {
        match self {
            Enum::BorrowedSpecificType(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::IntoVariant<VariantC<'l>> for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn into_variant(self) -> Result<VariantC<'l>, Self> {
        match self {
            Enum::OwnedWithGenericLifetime(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::IntoVariant<<VariantU as HasT>::T>
for Enum<'l, N, T>
where
    T: HasAssoc,
{
    fn into_variant(self) -> Result<<VariantU as HasT>::T, Self> {
        match self {
            Enum::OwnedAssocTypeOfSpecificTypeBehindCast(inner, ..) => Ok(inner),
            err => Err(err),
        }
    }
}
impl<'l, const N: usize, T> ::enumcapsulate::VariantDowncast for Enum<'l, N, T>
where
    T: HasAssoc,
{}
fn main() {}
