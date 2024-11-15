use std::collections::HashSet;

use syn::visit::Visit;

pub struct TypeVisitor<'ast> {
    const_param_idents: HashSet<&'ast syn::Ident>,
    type_param_idents: HashSet<&'ast syn::Ident>,

    uses_const_param: bool,
    uses_type_param: bool,
}

impl<'ast> TypeVisitor<'ast> {
    pub fn new(generics: &'ast syn::Generics) -> Self {
        Self {
            const_param_idents: generics.const_params().map(|param| &param.ident).collect(),
            type_param_idents: generics.type_params().map(|param| &param.ident).collect(),
            uses_const_param: false,
            uses_type_param: false,
        }
    }

    pub fn uses_const_or_type_param(self) -> bool {
        self.uses_const_param || self.uses_type_param
    }
}

impl<'ast> Visit<'ast> for TypeVisitor<'ast> {
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        if node.qself.is_none() {
            let path_segments = &node.path.segments;
            let first_capitalized_segment = path_segments.iter().find(|&segment| {
                let ident_name = segment.ident.to_string();
                let first_char: char = ident_name.chars().next().unwrap();
                first_char.is_uppercase()
            });

            if let Some(path_segment) = first_capitalized_segment {
                let ident = &path_segment.ident;

                if self.type_param_idents.contains(ident) {
                    self.uses_type_param = true;
                } else if self.const_param_idents.contains(ident) {
                    self.uses_const_param = true;
                }
            }
        }
        syn::visit::visit_type_path(self, node);
    }
}
