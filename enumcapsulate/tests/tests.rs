#[test]
pub fn pass() {
    tryexpand::expand_checking([
        // FromVariant
        "tests/expand-macros/from_variant/pass/**/*.rs",
        // AsVariantRef
        "tests/expand-macros/as_variant_ref/pass/**/*.rs",
        // AsVariantMut
        "tests/expand-macros/as_variant_mut/pass/**/*.rs",
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
    ]);
}
