#[derive(Clone, Default)]
pub(crate) struct EnumConfig {}

impl EnumConfig {
    pub(crate) fn from_enum(item_enum: &syn::ItemEnum) -> Result<Self, syn::Error> {
        let this = Self::default();

        let _ = item_enum;

        // parse_enumcapsulate_attrs(&item_enum.attrs, |meta| {
        //     Ok(())
        // })?;

        Ok(this)
    }
}
