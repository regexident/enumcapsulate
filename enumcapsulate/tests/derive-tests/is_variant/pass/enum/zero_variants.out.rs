use enumcapsulate::derive::IsVariant;
pub enum Enum {}
impl ::enumcapsulate::IsVariant for Enum {
    fn is_variant<T>(&self) -> bool
    where
        T: 'static + ?Sized,
    {
        ::core::panicking::panic("internal error: entered unreachable code")
    }
}
fn main() {}
