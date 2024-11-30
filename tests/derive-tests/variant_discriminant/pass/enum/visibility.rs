use enumcapsulate::VariantDiscriminant;

#[derive(VariantDiscriminant)]
pub enum PubEnum {}

#[derive(VariantDiscriminant)]
pub(crate) enum PubCrateEnum {}

#[derive(VariantDiscriminant)]
pub(self) enum PubSelfEnum {}

fn main() {}
