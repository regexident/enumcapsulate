use enumcapsulate::derive::IntoVariant;

#[derive(IntoVariant)]
pub union Union {
    field: (),
}

fn main() {}
