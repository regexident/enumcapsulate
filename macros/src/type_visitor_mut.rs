use syn::visit_mut::VisitMut;

pub struct TypeVisitorMut {
    replace_lifetimes_with_static: bool,
}

impl TypeVisitorMut {
    pub fn new() -> Self {
        Self {
            replace_lifetimes_with_static: false,
        }
    }

    pub fn replace_lifetimes_with_static(mut self) -> Self {
        self.replace_lifetimes_with_static = true;
        self
    }
}

impl Default for TypeVisitorMut {
    fn default() -> Self {
        Self::new()
    }
}

impl VisitMut for TypeVisitorMut {
    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        lifetime.ident = quote::format_ident!("static");
    }
}
