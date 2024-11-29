use syn::parse_quote;

use crate::config::*;

mod macro_idents {
    use super::*;

    #[test]
    fn parses_no_list() -> Result<(), syn::Error> {
        let attr: syn::Attribute = syn::parse_quote! {
            #[namespace(no_list)]
        };

        let mut actual: Vec<syn::Ident> = vec![];

        attr.parse_nested_meta(|meta| {
            assert!(meta.path.is_ident("no_list"));

            actual.extend(parse_idents_from_meta_list(&meta)?);

            Ok(())
        })?;

        assert!(actual.is_empty());

        Ok(())
    }

    #[test]
    fn parses_empty_list() -> Result<(), syn::Error> {
        let attr: syn::Attribute = syn::parse_quote! {
            #[namespace(empty_list())]
        };

        let mut actual: Vec<syn::Ident> = vec![];

        attr.parse_nested_meta(|meta| {
            assert!(meta.path.is_ident("empty_list"));

            actual.extend(parse_idents_from_meta_list(&meta)?);

            Ok(())
        })?;

        assert!(actual.is_empty());

        Ok(())
    }

    #[test]
    fn parses_non_empty_list() -> Result<(), syn::Error> {
        let attr: syn::Attribute = syn::parse_quote! {
            #[namespace(non_empty_list(Foo, Bar, Baz))]
        };

        let mut actual: Vec<syn::Ident> = vec![];

        attr.parse_nested_meta(|meta| {
            assert!(meta.path.is_ident("non_empty_list"));

            actual.extend(parse_idents_from_meta_list(&meta)?);

            Ok(())
        })?;

        let expected: Vec<syn::Ident> = vec![
            parse_quote! { Foo },
            parse_quote! { Bar },
            parse_quote! { Baz },
        ];

        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn accepts_valid_list() -> Result<(), syn::Error> {
        let idents: Vec<syn::Ident> = vec![parse_quote! { Foo }, parse_quote! { Bar }];

        let recognized: Vec<&str> = vec!["Foo", "Bar"];
        ensure_only_recognized_ident_names(&idents, &recognized)?;

        let conflicting: Vec<syn::Ident> = vec![parse_quote! { Baz }, parse_quote! { Blee }];
        ensure_no_conflicting_idents(&idents, &conflicting)?;

        Ok(())
    }

    #[test]
    fn detects_unrecognized_idents() -> Result<(), syn::Error> {
        let idents: Vec<syn::Ident> = vec![parse_quote! { Foo }, parse_quote! { Unrecognized }];

        let recognized: Vec<&str> = vec!["Foo", "Bar"];
        let error = ensure_only_recognized_ident_names(&idents, &recognized)
            .err()
            .unwrap();

        assert_eq!(error.to_string(), "unrecognized macro derive");

        Ok(())
    }

    #[test]
    fn detects_conflicting_idents() -> Result<(), syn::Error> {
        let idents: Vec<syn::Ident> = vec![parse_quote! { Foo }, parse_quote! { Bar }];

        let conflicting: Vec<syn::Ident> = vec![parse_quote! { Bar }];
        let error = ensure_no_conflicting_idents(&idents, &conflicting)
            .err()
            .unwrap();

        assert_eq!(error.to_string(), "conflicting macro derive");

        Ok(())
    }
}

mod enum_config {
    use super::*;

    #[test]
    fn accepts_empty_attrs() -> Result<(), syn::Error> {
        let item: syn::ItemEnum = parse_quote! {
            enum Dummy {}
        };

        let config = config_for_enum(&item)?;

        assert_eq!(config.exclude, None);

        Ok(())
    }

    #[test]
    fn accepts_empty_exclude_attrs() -> Result<(), syn::Error> {
        let item: syn::ItemEnum = parse_quote! {
            #[enumcapsulate(exclude)]
            enum Dummy {}
        };

        let config = config_for_enum(&item)?;

        let actual: Vec<syn::Ident> = config.exclude.unwrap().idents;
        let expected: Vec<syn::Ident> = vec![];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn accepts_non_empty_exclude_attrs() -> Result<(), syn::Error> {
        let item: syn::ItemEnum = parse_quote! {
            #[enumcapsulate(exclude(AsVariant, IntoVariant))]
            enum Dummy {}
        };

        let config = config_for_enum(&item)?;

        let actual = config.exclude.unwrap().idents;
        let expected: Vec<syn::Ident> =
            vec![parse_quote! { AsVariant }, parse_quote! { IntoVariant }];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn rejects_unrecognized_exclude_attrs() -> Result<(), syn::Error> {
        let item: syn::ItemEnum = parse_quote! {
            #[enumcapsulate(exclude(IntoVariant, Unrecognized))]
            enum Dummy {}
        };

        let error = config_for_enum(&item).err().unwrap();

        assert_eq!(error.to_string(), "unrecognized macro derive");

        Ok(())
    }

    #[test]
    fn is_excluded() {
        let config = EnumConfig {
            exclude: Some(MacroSelectionConfig {
                idents: vec![parse_quote! { FromVariant }, parse_quote! { IntoVariant }],
            }),
        };

        assert_eq!(config.is_excluded("FromVariant"), true);
        assert_eq!(config.is_excluded("IntoVariant"), true);
        assert_eq!(config.is_excluded("AsVariant"), false);
    }
}

mod variant_config {
    use super::*;

    #[test]
    fn accepts_empty_attrs() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            Dummy(bool)
        };

        let config = config_for_variant(&variant)?;

        assert_eq!(config.exclude, None);
        assert_eq!(config.include, None);
        assert_eq!(config.field, None);

        Ok(())
    }

    #[test]
    fn accepts_field_index_attr_for_unnamed_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(index = 2))]
            Dummy(i8, i16, i32, i64)
        };

        let config = config_for_variant(&variant)?;

        assert_eq!(config.exclude, None);
        assert_eq!(config.include, None);
        assert_eq!(config.field, Some(VariantFieldConfig::Index(2)));

        Ok(())
    }

    #[test]
    fn accepts_field_index_attr_for_named_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(index = 2))]
            Dummy { foo: i8, bar: i16, baz: i32, blee: i64 }
        };

        let config = config_for_variant(&variant)?;

        assert_eq!(config.exclude, None);
        assert_eq!(config.include, None);
        assert_eq!(config.field, Some(VariantFieldConfig::Index(2)));

        Ok(())
    }

    #[test]
    fn rejects_invalid_field_index_attr_for_unnamed_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(index = 42))]
            Dummy(i8, i16, i32, i64)
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "field index out of bounds");

        Ok(())
    }

    #[test]
    fn rejects_invalid_field_index_attr_for_named_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(index = 42))]
            Dummy { foo: i8, bar: i16, baz: i32, blee: i64 }
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "field index out of bounds");

        Ok(())
    }

    #[test]
    fn rejects_field_name_attr_for_unnamed_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(name = "bar"))]
            Dummy(i8, i16, i32, i64)
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "no named fields in variant");

        Ok(())
    }

    #[test]
    fn accepts_field_name_attr_for_named_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(name = "bar"))]
            Dummy { foo: i8, bar: i16, baz: i32, blee: i64 }
        };

        let config = config_for_variant(&variant)?;

        assert_eq!(config.exclude, None);
        assert_eq!(config.include, None);
        assert_eq!(
            config.field,
            Some(VariantFieldConfig::Name("bar".to_owned()))
        );

        Ok(())
    }

    #[test]
    fn rejects_invalid_field_name_attr_for_named_fields() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(field(name = "invalid"))]
            Dummy { foo: i8, bar: i16, baz: i32, blee: i64 }
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "field not found in variant");

        Ok(())
    }

    #[test]
    fn accepts_empty_exclude_include_attrs() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(exclude)]
            #[enumcapsulate(include)]
            Dummy(bool)
        };

        let config = config_for_variant(&variant)?;

        assert_eq!(
            config.exclude.unwrap(),
            MacroSelectionConfig { idents: vec![] }
        );
        assert_eq!(
            config.include.unwrap(),
            MacroSelectionConfig { idents: vec![] }
        );
        assert_eq!(config.field, None);

        Ok(())
    }

    #[test]
    fn accepts_non_empty_exclude_include_attrs() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(exclude(From, TryInto))]
            #[enumcapsulate(include(FromVariant, IntoVariant))]
            Dummy(bool)
        };

        let config = config_for_variant(&variant)?;

        assert_eq!(
            config.exclude.unwrap(),
            MacroSelectionConfig {
                idents: vec![parse_quote! { From }, parse_quote! { TryInto }]
            }
        );
        assert_eq!(
            config.include.unwrap(),
            MacroSelectionConfig {
                idents: vec![parse_quote! { FromVariant }, parse_quote! { IntoVariant }]
            }
        );

        Ok(())
    }

    #[test]
    fn rejects_unrecognized_exclude_attrs() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(exclude(IntoVariant, Unrecognized))]
            Dummy(bool)
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "unrecognized macro derive");

        Ok(())
    }

    #[test]
    fn rejects_unrecognized_include_attrs() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(include(IntoVariant, Unrecognized))]
            Dummy(bool)
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "unrecognized macro derive");

        Ok(())
    }

    #[test]
    fn rejects_conflicting_exclude_include_attrs() -> Result<(), syn::Error> {
        let variant: syn::Variant = parse_quote! {
            #[enumcapsulate(exclude(From, TryInto))]
            #[enumcapsulate(include(TryInto, IntoVariant))]
            Dummy(bool)
        };

        let error = config_for_variant(&variant).err().unwrap();

        assert_eq!(error.to_string(), "conflicting macro derive");

        Ok(())
    }

    mod is_excluded {
        use super::*;

        #[test]
        fn no_enum_excludes() {
            let enum_config = EnumConfig { exclude: None };

            let config = VariantConfig {
                exclude: None,
                include: None,
                field: None,
            };

            assert_eq!(config.is_excluded("FromVariant", &enum_config), false);
            assert_eq!(config.is_excluded("IntoVariant", &enum_config), false);
            assert_eq!(config.is_excluded("AsVariant", &enum_config), false);
        }

        #[test]
        fn only_enum_excludes() {
            let enum_config = EnumConfig {
                exclude: Some(MacroSelectionConfig {
                    idents: vec![parse_quote! { AsVariant }],
                }),
            };

            let config = VariantConfig {
                exclude: None,
                include: None,
                field: None,
            };

            assert_eq!(config.is_excluded("FromVariant", &enum_config), false);
            assert_eq!(config.is_excluded("IntoVariant", &enum_config), false);
            assert_eq!(config.is_excluded("AsVariant", &enum_config), true);
        }

        #[test]
        fn blanket_overridden_enum_excludes() {
            let enum_config = EnumConfig {
                exclude: Some(MacroSelectionConfig {
                    idents: vec![parse_quote! { AsVariant }],
                }),
            };

            let config = VariantConfig {
                exclude: None,
                include: Some(MacroSelectionConfig { idents: vec![] }),
                field: None,
            };

            assert_eq!(config.is_excluded("FromVariant", &enum_config), false);
            assert_eq!(config.is_excluded("IntoVariant", &enum_config), false);
            assert_eq!(config.is_excluded("AsVariant", &enum_config), false);
        }

        #[test]
        fn selective_overridden_enum_excludes() {
            let enum_config = EnumConfig {
                exclude: Some(MacroSelectionConfig {
                    idents: vec![parse_quote! { AsVariant }, parse_quote! { IntoVariant }],
                }),
            };

            let config = VariantConfig {
                exclude: None,
                include: Some(MacroSelectionConfig {
                    idents: vec![parse_quote! { AsVariant }],
                }),
                field: None,
            };

            assert_eq!(config.is_excluded("FromVariant", &enum_config), false);
            assert_eq!(config.is_excluded("IntoVariant", &enum_config), true);
            assert_eq!(config.is_excluded("AsVariant", &enum_config), false);
        }

        #[test]
        fn selective_overridden_variant_excludes() {
            let enum_config = EnumConfig { exclude: None };

            let config = VariantConfig {
                exclude: Some(MacroSelectionConfig { idents: vec![] }),
                include: Some(MacroSelectionConfig {
                    idents: vec![parse_quote! { AsVariant }],
                }),
                field: None,
            };

            assert_eq!(config.is_excluded("FromVariant", &enum_config), true);
            assert_eq!(config.is_excluded("IntoVariant", &enum_config), true);
            assert_eq!(config.is_excluded("AsVariant", &enum_config), false);
        }
    }
}
