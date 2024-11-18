use enumcapsulate::AsVariant;
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
#[automatically_derived]
impl ::core::clone::Clone for VariantT {
    #[inline]
    fn clone(&self) -> VariantT {
        VariantT
    }
}
impl HasAssoc for VariantT {
    type Assoc = ();
}
pub struct VariantU;
#[automatically_derived]
impl ::core::clone::Clone for VariantU {
    #[inline]
    fn clone(&self) -> VariantU {
        VariantU
    }
}
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
fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;
    let mut subject = Subject::Unit;
    let _: Option<VariantA> = subject.as_variant();
    let _: Option<&VariantB> = subject.as_variant();
    let _: Option<VariantC<'_>> = subject.as_variant();
    let _: Option<VariantC<'_>> = subject.as_variant();
    let _: Option<<VariantU as HasT>::T> = subject.as_variant();
}
