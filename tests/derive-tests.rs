mod from {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/from/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/from/fail/**/*.rs"]).expect_fail();
    }
}

mod try_into {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/try_into/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/try_into/fail/**/*.rs"]).expect_fail();
    }
}

mod from_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/from_variant/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/from_variant/fail/**/*.rs"]).expect_fail();
    }
}

mod as_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/as_variant/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/as_variant/fail/**/*.rs"]).expect_fail();
    }
}

mod as_variant_ref {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/as_variant_ref/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/as_variant_ref/fail/**/*.rs"]).expect_fail();
    }
}

mod as_variant_mut {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/as_variant_mut/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/as_variant_mut/fail/**/*.rs"]).expect_fail();
    }
}

mod into_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/into_variant/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/into_variant/fail/**/*.rs"]).expect_fail();
    }
}

mod variant_downcast {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/variant_downcast/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/derive-tests/variant_downcast/fail/**/*.rs"]).expect_fail();
    }
}

mod variant_discriminant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/variant_discriminant/pass/**/*.rs"]).and_check();
    }

    // There should be no failures for this derive macro;
}

mod encapsulate {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/encapsulate/pass/**/*.rs"]).and_check();
    }

    // Failures are already covered by the tests of the individual
    // derives that this umbrella derive delegates to.
    //
    // As such we only have to make sure in `pass()` that
    // it does actually derive what it says on the tin.
}

mod smoke {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/derive-tests/smoke/pass/**/*.rs"]).and_check();
    }

    // Failures are already covered by the tests of the individual
    // derives that this umbrella derive delegates to.
    //
    // As such we only have to make sure in `pass()` that
    // it does actually derive what it says on the tin.
}
