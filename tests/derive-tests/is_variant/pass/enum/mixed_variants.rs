use enumcapsulate::derive::IsVariant;
use enumcapsulate::IsVariant;

pub struct VariantA;
pub struct VariantB;

#[derive(IsVariant)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    TwoTupleFields(i32, u32),
    TwoStructFields {
        a: i32,
        b: u32,
    },
    #[enumcapsulate(exclude)]
    Excluded(bool),
}

fn check<T>()
where
    T: IsVariant,
{
}

fn main() {
    check::<Enum>();
}
