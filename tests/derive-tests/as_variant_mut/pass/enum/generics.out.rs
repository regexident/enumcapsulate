use enumcapsulate::AsVariantMut;
trait HasAssoc {
    type Assoc: Sized;
}
pub struct VariantA;
pub struct VariantB;
pub struct VariantC<'l>(&'l ());
pub struct VariantD<const N: usize>([(); N]);
pub struct VariantE<T>(T);
pub struct VariantF<'l, const N: usize, T>(&'l [T; N]);
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
fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;
    let mut subject = Subject::Unit;
    {
        let _: Option<&mut VariantA> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut &VariantB> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantC<'_>> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantC<'_>> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut <VariantU as HasT>::T> = subject.as_variant_mut();
    }
}
