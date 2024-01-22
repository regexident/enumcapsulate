#[test]
pub fn pass() {
    tryexpand::expand_checking([
        // From
        "tests/expand-macros/from/pass/**/*.rs",
        // TryInto
        "tests/expand-macros/try_into/pass/**/*.rs",
        // FromVariant
        "tests/expand-macros/from_variant/pass/**/*.rs",
        // AsVariantRef
        "tests/expand-macros/as_variant_ref/pass/**/*.rs",
        // AsVariantMut
        "tests/expand-macros/as_variant_mut/pass/**/*.rs",
        // AsVariant
        "tests/expand-macros/as_variant/pass/**/*.rs",
        // IntoVariant
        "tests/expand-macros/into_variant/pass/**/*.rs",
        // VariantDowncast
        "tests/expand-macros/variant_downcast/pass/**/*.rs",
    ]);
}

#[test]
pub fn fail() {
    tryexpand::expand_fail([
        // From
        "tests/expand-macros/from/fail/**/*.rs",
        // TryInto
        "tests/expand-macros/try_into/fail/**/*.rs",
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
