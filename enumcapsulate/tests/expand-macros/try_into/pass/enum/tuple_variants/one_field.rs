use enumcapsulate::derive::TryInto;

pub struct VariantA;
pub struct VariantB;

#[derive(TryInto)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn main() {
    let subject = Enum::VariantA(VariantA);

    let _: Result<VariantA, Enum> = subject.try_into();
}
