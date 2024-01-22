use enumcapsulate::derive::From;

pub struct VariantA;

#[derive(From)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(i32, u32),
}
