use enumcapsulate::FromVariant;

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

#[derive(FromVariant)]
pub enum Enum<'l, const N: usize, T>
where
    T: HasAssoc,
{
    Unit,
    // We expect this variant to get derived:
    OwnedSpecificType(VariantA),
    // We expect this variant to get derived:
    BorrowedSpecificType(&'l VariantB),
    // We expect this variant to get derived:
    OwnedWithGenericLifetime(VariantC<'l>),
    // We expect this variant to NOT get derived (due to its use of a const param):
    OwnedGenericTypeWithConstParam(VariantD<N>),
    // We expect this variant to NOT get derived (due to its use of a type param):
    OwnedGenericTypeWithTypeParam(VariantE<T>),
    // We expect this variant to NOT get derived (due to its use of const and type params):
    OwnedGenericTypeWithMixedParams(VariantF<'l, N, T>),
    // We expect this variant to NOT get derived (due to its use of a type param):
    OwnedGenericParam(T),
    // We expect this variant to NOT get derived (due to its use of a type param):
    BorrowedGenericParam(&'l T),
    // We expect this variant to NOT get derived (due to its use of a type param):
    OwnedAssocTypeOfGenericParam(T::Assoc),
    // We expect this variant to NOT get derived (due to its use of a type param):
    OwnedAssocTypeOfGenericParamBehindCast(<T as HasAssoc>::Assoc),
    // We expect this variant to get derived:
    OwnedAssocTypeOfSpecificTypeBehindCast(<VariantU as HasT>::T),
}

fn main() {
    type Subject<'x> = Enum<'x, 42, VariantT>;

    let _: Subject<'_> = Enum::from_variant(VariantA);
    let _: Subject<'_> = Enum::from_variant(&VariantB);
    let _: Subject<'_> = Enum::from_variant(VariantC(&42_i8));
    let _: Subject<'_> = Enum::from_variant(42_i64);
}
