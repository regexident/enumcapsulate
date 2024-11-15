use enumcapsulate::IntoVariant;

#[derive(IntoVariant)]
pub union Union {
    field: (),
}

fn main() {}
