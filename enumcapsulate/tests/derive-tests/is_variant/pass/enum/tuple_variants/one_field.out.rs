use enumcapsulate::derive::IsVariant;
use enumcapsulate::IsVariant;
pub struct VariantA;
pub struct VariantB;
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}
impl ::enumcapsulate::IsVariant for Enum {
    fn is_variant<T>(&self) -> bool
    where
        T: 'static + ?Sized,
    {
        use ::std::any::TypeId;
        #[inline]
        pub fn type_id_of_val<T: 'static + ?Sized>(_val: &T) -> TypeId {
            TypeId::of::<T>()
        }
        let type_id = TypeId::of::<T>();
        match self {
            Enum::VariantA(variant) => type_id_of_val(variant) == type_id,
            Enum::VariantB(variant) => type_id_of_val(variant) == type_id,
        }
    }
}
fn check<T>()
where
    T: IsVariant,
{}
fn main() {
    check::<Enum>();
    check::<Enum>();
}
