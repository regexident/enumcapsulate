use enumcapsulate::AsVariantRef;

#[derive(AsVariantRef)]
pub union Union {
    field: (),
}

fn main() {}
