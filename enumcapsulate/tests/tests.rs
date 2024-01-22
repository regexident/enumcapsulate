#[test]
pub fn pass() {
    tryexpand::expand_checking([
        // FromVariant
        "tests/expand-macros/from_variant/pass/**/*.rs",
        // AsVariantRef
        "tests/expand-macros/as_variant_ref/pass/**/*.rs",
        // AsVariantMut
        "tests/expand-macros/as_variant_mut/pass/**/*.rs",
        // IntoVariant
        "tests/expand-macros/into_variant/pass/**/*.rs",
        // VariantDowncast
        "tests/expand-macros/variant_downcast/pass/**/*.rs",
    ]);
}

#[test]
pub fn fail() {
    tryexpand::expand_fail([
        // FromVariant
        "tests/expand-macros/from_variant/fail/**/*.rs",
        // AsVariantRef
        "tests/expand-macros/as_variant_ref/fail/**/*.rs",
        // AsVariantMut
        "tests/expand-macros/as_variant_mut/fail/**/*.rs",
        // IntoVariant
        "tests/expand-macros/into_variant/fail/**/*.rs",
        // VariantDowncast
        "tests/expand-macros/variant_downcast/fail/**/*.rs",
    ]);
}
