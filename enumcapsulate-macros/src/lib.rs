use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::tokenstream;

mod enum_deriver;
mod utils;
