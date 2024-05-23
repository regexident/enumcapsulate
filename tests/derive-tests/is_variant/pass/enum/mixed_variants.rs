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
    OneStructField { variant: VariantB },
}

fn check<T>()
where
    T: IsVariant,
{
}

fn main() {
    check::<Enum>();
}
