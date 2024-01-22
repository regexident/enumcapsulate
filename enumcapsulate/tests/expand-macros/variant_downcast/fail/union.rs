use enumcapsulate::derive::AsVariant;

#[derive(AsVariant)]
pub union Union {
    field: (),
}

fn main() {}
