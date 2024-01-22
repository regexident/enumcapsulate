use enumcapsulate::{
    derive::Encapsulate, AsVariantMut, AsVariantRef, IntoVariant, VariantDowncast,
};
pub enum Enum {}
impl ::enumcapsulate::VariantDowncast for Enum {}
impl ::enumcapsulate::IsVariant for Enum {
    fn is_variant<T>(&self) -> bool
    where
        T: 'static + ?Sized,
    {
        ::core::panicking::panic("internal error: entered unreachable code")
    }
}
fn check<T>()
where
    T: VariantDowncast,
{}
fn main() {
    check::<Enum>();
}
