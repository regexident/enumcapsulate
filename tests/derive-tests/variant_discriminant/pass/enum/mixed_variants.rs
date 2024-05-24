use enumcapsulate::derive::VariantDiscriminant;

pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;

#[derive(VariantDiscriminant)]
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
}

fn main() {
    use enumcapsulate::VariantDiscriminant;

    fn check<T>()
    where
        T: VariantDiscriminant,
    {
    }

    check::<Enum>();
}
