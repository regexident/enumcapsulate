use enumcapsulate::IntoVariant;

#[derive(IntoVariant)]
pub struct Struct {
    field: (),
}

fn main() {}
