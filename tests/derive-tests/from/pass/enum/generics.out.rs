use enumcapsulate::From;
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
fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;
    let _: Subject<'_> = Enum::from(VariantA);
    let _: Subject<'_> = Enum::from(&VariantB);
    let _: Subject<'_> = Enum::from(VariantC(&42_i8));
    let _: Subject<'_> = Enum::from(42_i64);
}