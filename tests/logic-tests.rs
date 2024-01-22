#![allow(dead_code)]

#[derive(Clone, PartialEq, Debug)]
struct VariantA;

#[derive(Clone, PartialEq, Debug)]
struct VariantB;

#[test]
fn from() {
    use enumcapsulate_macros::From;

    #[derive(PartialEq, Debug, From)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    assert_eq!(Enum::from(VariantA), Enum::VariantA(VariantA));
    assert_eq!(Enum::from(VariantB), Enum::VariantB(VariantB));
}

mod into {
    use super::*;

    use enumcapsulate_macros::TryInto;

    #[derive(Clone, PartialEq, Debug, TryInto)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    #[test]
    fn returns_ok_for_match() {
        let subject: Enum = Enum::VariantA(VariantA);
        let result: Result<VariantA, Enum> = subject.clone().try_into();
        assert_eq!(result, Ok(VariantA));
    }

    #[test]
    fn returns_err_for_mismatch() {
        let subject: Enum = Enum::VariantA(VariantA);
        let result: Result<VariantB, Enum> = subject.clone().try_into();
        assert_eq!(result, Err(subject));
    }
}

#[test]
fn from_variant() {
    use enumcapsulate::FromVariant;
    use enumcapsulate_macros::FromVariant;

    #[derive(PartialEq, Debug, FromVariant)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    assert_eq!(Enum::from_variant(VariantA), Enum::VariantA(VariantA));
    assert_eq!(Enum::from_variant(VariantB), Enum::VariantB(VariantB));
}

mod into_variant {
    use super::*;

    use enumcapsulate::IntoVariant;
    use enumcapsulate_macros::IntoVariant;

    #[derive(Clone, PartialEq, Debug, IntoVariant)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    #[test]
    fn returns_ok_for_match() {
        let subject: Enum = Enum::VariantA(VariantA);
        let result: Result<VariantA, Enum> = subject.clone().into_variant();
        assert_eq!(result, Ok(VariantA));
    }

    #[test]
    fn returns_err_for_mismatch() {
        let subject: Enum = Enum::VariantA(VariantA);
        let result: Result<VariantB, Enum> = subject.clone().into_variant();
        assert_eq!(result, Err(subject));
    }
}

mod as_variant_ref {
    use super::*;

    use enumcapsulate::AsVariantRef;
    use enumcapsulate_macros::AsVariantRef;

    #[derive(PartialEq, Debug, AsVariantRef)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    #[test]
    fn returns_some_for_match() {
        let subject: Enum = Enum::VariantA(VariantA);
        let option: Option<&VariantA> = subject.as_variant_ref();
        assert_eq!(option, Some(&VariantA));
    }

    #[test]
    fn returns_none_for_mismatch() {
        let subject: Enum = Enum::VariantA(VariantA);
        let option: Option<&VariantB> = subject.as_variant_ref();
        assert_eq!(option, None);
    }
}

mod as_variant_mut {
    use super::*;

    use enumcapsulate::AsVariantMut;
    use enumcapsulate_macros::AsVariantMut;

    #[derive(PartialEq, Debug, AsVariantMut)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    #[test]
    fn returns_some_for_match() {
        let mut subject: Enum = Enum::VariantA(VariantA);
        let option: Option<&mut VariantA> = subject.as_variant_mut();
        assert_eq!(option, Some(&mut VariantA));
    }

    #[test]
    fn returns_none_for_mismatch() {
        let mut subject: Enum = Enum::VariantA(VariantA);
        let option: Option<&mut VariantB> = subject.as_variant_mut();
        assert_eq!(option, None);
    }
}

mod as_variant {
    use super::*;

    use enumcapsulate::AsVariantRef;
    use enumcapsulate_macros::AsVariantRef;

    #[derive(PartialEq, Debug, AsVariantRef)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    #[test]
    fn returns_some_for_match() {
        let subject: Enum = Enum::VariantA(VariantA);
        let option: Option<VariantA> = subject.as_variant();
        assert_eq!(option, Some(VariantA));
    }

    #[test]
    fn returns_none_for_mismatch() {
        let subject: Enum = Enum::VariantA(VariantA);
        let option: Option<VariantB> = subject.as_variant();
        assert_eq!(option, None);
    }
}

#[test]
fn is_variant() {
    use enumcapsulate::IsVariant;
    use enumcapsulate_macros::IsVariant;

    #[derive(PartialEq, Debug, IsVariant)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    assert_eq!(Enum::VariantA(VariantA).is_variant::<VariantA>(), true);
    assert_eq!(Enum::VariantB(VariantB).is_variant::<VariantA>(), false);
}

#[test]
fn variant_discriminant() {
    use enumcapsulate::VariantDiscriminant;
    use enumcapsulate_macros::VariantDiscriminant;

    #[derive(PartialEq, Debug, VariantDiscriminant)]
    enum Enum {
        VariantA(VariantA),
        VariantB(VariantB),
    }

    assert_eq!(
        Enum::VariantA(VariantA).variant_discriminant(),
        EnumDiscriminant::VariantA
    );
    assert_eq!(
        Enum::VariantB(VariantB).variant_discriminant(),
        EnumDiscriminant::VariantB
    );
}
