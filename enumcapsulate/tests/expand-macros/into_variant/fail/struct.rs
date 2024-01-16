use enumcapsulate::derive::IntoVariant;

#[derive(IntoVariant)]
pub struct Struct {
    field: (),
}

fn main() {}
