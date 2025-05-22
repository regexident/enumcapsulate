use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
pub enum VariantWithLifetime<'a> {
    Variant(&'a ()),
}

// #[derive(VariantDiscriminant)]
// pub enum GenericVariantWithLifetime<'a, T> {
//     Variant(&'a T),
// }

#[derive(VariantDiscriminant)]
pub enum EnumWithLifetime<'a> {
    #[enumcapsulate(discriminant(nested))]
    VariantA(VariantWithLifetime<'a>),
    // #[enumcapsulate(discriminant(nested))]
    // VariantB { b: VariantWithLifetime<'a> },
    // #[enumcapsulate(field = 0, discriminant(nested = GenericVariantWithLifetimeDiscriminant))]
    // VariantC(GenericVariantWithLifetime<'a, T>),
    // #[enumcapsulate(field = "d", discriminant(nested = GenericVariantWithLifetimeDiscriminant))]
    // VariantD {
    //     d: GenericVariantWithLifetime<'a, T>,
    // },
}

fn main() {}
