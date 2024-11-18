use enumcapsulate::IsVariant;

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
    // This associated type intentionally has the same name
    // as the generic type param of the enum subject.
    //
    // The point of this is to detect false positives
    // from the generic param detection, which should falsely
    // detect `VariantU::T` as a use of the type param `T`:
    type T: Sized;
}

impl HasT for VariantU {
    type T = i64;
}

#[derive(IsVariant)]
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

fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;

    let subject = Subject::Unit;

    let _: bool = subject.is_variant::<VariantA>();
    let _: bool = subject.is_variant::<&'_ VariantB>();
    let _: bool = subject.is_variant::<VariantC<'_>>();
    let _: bool = subject.is_variant::<VariantD<42>>();
    let _: bool = subject.is_variant::<VariantE<VariantT>>();
    let _: bool = subject.is_variant::<VariantF<'_, 42, VariantT>>();
    let _: bool = subject.is_variant::<VariantT>();
    let _: bool = subject.is_variant::<&'_ VariantT>();
    let _: bool = subject.is_variant::<<VariantT as HasAssoc>::Assoc>();
    let _: bool = subject.is_variant::<<VariantU as HasT>::T>();
}
