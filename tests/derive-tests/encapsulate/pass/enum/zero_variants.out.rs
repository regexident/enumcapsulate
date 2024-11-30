use enumcapsulate::{
    AsVariantMut, AsVariantRef, Encapsulate, IntoVariant, VariantDowncast,
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
