use enumcapsulate::TryInto;
trait HasAssoc {
    type Assoc: Sized;
}
pub struct VariantA;
pub struct VariantB;
pub struct VariantC<'l>(&'l i8);
pub struct VariantD<const N: usize>([i16; N]);
pub struct VariantE<T>(T);
pub struct VariantF<'l, const N: usize, T>(&'l [T; N]);
pub struct VariantT;
impl HasAssoc for VariantT {
    type Assoc = i32;
}
pub struct VariantU;
trait HasT {
    type T: Sized;
}
impl HasT for VariantU {
    type T = i64;
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
fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;
    {
        let subject = Subject::Unit;
        let _: Result<VariantA, Subject<'_>> = subject.try_into();
    }
    {
        let subject = Subject::Unit;
        let _: Result<&'_ VariantB, Subject<'_>> = subject.try_into();
    }
    {
        let subject = Subject::Unit;
        let _: Result<VariantC<'_>, Subject<'_>> = subject.try_into();
    }
    {
        let subject = Subject::Unit;
        let _: Result<<VariantU as HasT>::T, Subject<'_>> = subject.try_into();
    }
}
