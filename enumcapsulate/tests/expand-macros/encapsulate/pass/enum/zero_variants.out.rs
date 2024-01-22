use enumcapsulate::{
    derive::Encapsulate, AsVariantMut, AsVariantRef, IntoVariant, VariantDowncast,
};
pub enum Enum {}
impl ::enumcapsulate::VariantDowncast for Enum {}
fn check<T>()
where
    T: VariantDowncast,
{}
fn main() {
    check::<Enum>();
}
