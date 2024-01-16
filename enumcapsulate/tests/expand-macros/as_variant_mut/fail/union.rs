use enumcapsulate::derive::AsVariantMut;

#[derive(AsVariantMut)]
pub union Union {
    field: (),
}

fn main() {}
