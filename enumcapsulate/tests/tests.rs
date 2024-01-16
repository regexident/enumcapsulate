#[test]
pub fn pass() {
    tryexpand::expand_checking([
        // FromVariant
        "tests/expand-macros/from_variant/pass/**/*.rs",
    ]);
}

#[test]
pub fn fail() {
    tryexpand::expand_fail([
        // FromVariant
        "tests/expand-macros/from_variant/fail/**/*.rs",
    ]);
}
