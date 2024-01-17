mod from {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/from/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/from/fail/**/*.rs"]);
    }
}

mod try_into {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/try_into/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/try_into/fail/**/*.rs"]);
    }
}

mod from_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/from_variant/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/from_variant/fail/**/*.rs"]);
    }
}

mod as_variant_ref {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/as_variant_ref/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/as_variant_ref/fail/**/*.rs"]);
    }
}

mod as_variant_mut {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/as_variant_mut/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/as_variant_mut/fail/**/*.rs"]);
    }
}

mod as_variant {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/as_variant/pass/**/*.rs"]);
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
        tryexpand::expand_checking(["tests/expand-macros/into_variant/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/into_variant/fail/**/*.rs"]);
    }
}

mod variant_downcast {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/variant_downcast/pass/**/*.rs"]);
    }

    #[test]
    pub fn fail() {
        tryexpand::expand_fail(["tests/expand-macros/variant_downcast/fail/**/*.rs"]);
    }
}

mod encapsulate {
    #[test]
    pub fn pass() {
        tryexpand::expand_checking(["tests/expand-macros/encapsulate/pass/**/*.rs"]);
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
