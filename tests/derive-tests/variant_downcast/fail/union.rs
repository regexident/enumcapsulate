use enumcapsulate::AsVariant;

#[derive(AsVariant)]
pub union Union {
    field: (),
}

fn main() {}
