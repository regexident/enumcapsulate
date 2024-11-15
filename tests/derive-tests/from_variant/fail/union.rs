use enumcapsulate::FromVariant;

#[derive(FromVariant)]
pub union Union {
    field: (),
}

fn main() {}
