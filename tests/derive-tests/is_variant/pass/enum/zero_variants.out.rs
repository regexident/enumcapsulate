use enumcapsulate::derive::IsVariant;
pub enum Enum {}
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
            _ => false,
        }
    }
}
fn main() {}
