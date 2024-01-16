use enumcapsulate::derive::AsVariantMut;

pub struct VariantA;
pub struct VariantB;

#[derive(AsVariantMut)]
pub enum Enum {
    VariantA(VariantA),
    VariantB(VariantB),
}

fn main() {
    use enumcapsulate::AsVariantMut;

    let mut subject = Enum::VariantA(VariantA);

    {
        let _: Option<&mut VariantA> = subject.as_variant_mut();
    }
    {
        let _: Option<&mut VariantB> = subject.as_variant_mut();
    }
}
