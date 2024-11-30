use crate::attr;

mod exclude;
mod for_enum;
mod for_variant;

pub(crate) use self::{exclude::*, for_enum::*, for_variant::*};

pub(crate) fn parse_enumcapsulate_attrs(
    attrs: &[syn::Attribute],
    logic: impl FnMut(syn::meta::ParseNestedMeta) -> Result<(), syn::Error>,
) -> Result<(), syn::Error> {
    let mut logic = logic;

    for attr in attrs {
        if !attr.path().is_ident(attr::NAMESPACE) {
            continue;
        }

        attr.parse_nested_meta(&mut logic)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
