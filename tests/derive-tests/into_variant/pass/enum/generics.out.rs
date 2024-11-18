use enumcapsulate::IntoVariant;
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
fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;
    {
        let subject = Subject::Unit;
        let _: Result<VariantA, Subject<'_>> = subject.into_variant();
    }
    {
        let subject = Subject::Unit;
        let _: Result<&'_ VariantB, Subject<'_>> = subject.into_variant();
    }
    {
        let subject = Subject::Unit;
        let _: Result<VariantC<'_>, Subject<'_>> = subject.into_variant();
    }
    {
        let subject = Subject::Unit;
        let _: Result<<VariantU as HasT>::T, Subject<'_>> = subject.into_variant();
    }
}
