use enumcapsulate::derive::FromVariant;

#[derive(FromVariant)]
pub union Union {
    field: (),
}

fn main() {}
