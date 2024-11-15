use enumcapsulate::IsVariant;
pub struct VariantA;
pub struct VariantB;
pub struct VariantC;
pub struct VariantD;
pub enum Enum {
    Unit,
    ZeroTupleFields(),
    ZeroStructFields {},
    OneTupleField(VariantA),
    OneStructField { variant: VariantB },
    TwoTupleFields(i32, u32),
    TwoStructFields { a: i32, b: u32 },
    #[enumcapsulate(exclude)]
    Excluded(bool),
    #[enumcapsulate(include(field = 1))]
    IncludedTuple(i8, VariantC),
    #[enumcapsulate(include(field = "variant"))]
    IncludedStruct { value: u8, variant: VariantD },
}
impl ::enumcapsulate::IsVariant for Enum {
    fn is_variant<T>(&self) -> bool
    where
        T: 'static + ?Sized,
    {
        use ::std::any::TypeId;
        #[inline]
        pub fn type_id_of_val<T: 'static + ?Sized>(_val: &T) -> TypeId {
            TypeId::of::<T>()
        }
        let type_id = TypeId::of::<T>();
        match self {
            Enum::OneTupleField(inner, ..) => type_id_of_val(inner) == type_id,
            Enum::OneStructField { variant: inner, .. } => {
                type_id_of_val(inner) == type_id
            }
            Enum::IncludedTuple(_, inner, ..) => type_id_of_val(inner) == type_id,
            Enum::IncludedStruct { variant: inner, .. } => {
                type_id_of_val(inner) == type_id
            }
            _ => false,
        }
    }
}
fn check<T>()
where
    T: IsVariant,
{}
fn main() {
    check::<Enum>();
}
