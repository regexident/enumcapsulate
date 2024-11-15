use enumcapsulate::AsVariantMut;

#[derive(AsVariantMut)]
pub union Union {
    field: (),
}

fn main() {}
