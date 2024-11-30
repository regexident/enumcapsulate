use enumcapsulate::Encapsulate;

#[derive(Clone)]
pub struct VariantA;
#[derive(Clone)]
pub struct VariantB;
#[derive(Clone)]
pub struct VariantC;
#[derive(Clone)]
pub struct VariantD;

#[derive(Encapsulate)]
#[enumcapsulate(exclude(From, TryInto))]
pub enum Enum {
    OneTupleField(VariantA),
    OneStructField {
        variant: VariantB,
    },
    #[enumcapsulate(exclude)]
    ExcludedWildcard(VariantC),
    #[enumcapsulate(exclude(FromVariant, AsVariant))]
    ExcludedSelective(VariantD),
}

fn main() {}
