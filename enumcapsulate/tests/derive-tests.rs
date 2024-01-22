mod from {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/from/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/from/fail/**/*.rs"]).expect_fail();
    }
}

mod try_into {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/try_into/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/try_into/fail/**/*.rs"]).expect_fail();
    }
}

mod from_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/from_variant/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/from_variant/fail/**/*.rs"]).expect_fail();
    }
}

mod as_variant_ref {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/as_variant_ref/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/as_variant_ref/fail/**/*.rs"]).expect_fail();
    }
}

mod as_variant_mut {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/as_variant_mut/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/as_variant_mut/fail/**/*.rs"]).expect_fail();
    }
}

mod as_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/as_variant/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        // the failures are already covered by the tests of the individual
        // derives that this umbrella derive delegates to.
        //
        // As such we only have to make sure in `pass()` that
        // it does actually derive what it says on the tin.
    }
}

mod into_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/into_variant/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/into_variant/fail/**/*.rs"]).expect_fail();
    }
}

mod variant_downcast {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/variant_downcast/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        tryexpand::expand(["tests/expand-macros/variant_downcast/fail/**/*.rs"]).expect_fail();
    }
}

mod encapsulate {
    #[test]
    pub fn pass() {
        tryexpand::expand(["tests/expand-macros/encapsulate/pass/**/*.rs"]).and_check();
    }

    #[test]
    pub fn fail() {
        // the failures are already covered by the tests of the individual
        // derives that this umbrella derive delegates to.
        //
        // As such we only have to make sure in `pass()` that
        // it does actually derive what it says on the tin.
    }
}
