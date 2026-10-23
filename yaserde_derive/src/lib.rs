#![recursion_limit = "256"]

// Required for Rust < 1.42
extern crate proc_macro;

mod common;
mod de;
mod primitives;
mod ser;
#[cfg(test)]
mod tests;

use primitives::{hexbinary_serde, primitive_serde, primitive_yaserde};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_derive(YaDeserialize, attributes(yaserde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as syn::DeriveInput);

  common::validate(&ast)
    .and_then(|()| de::expand_derive_deserialize(&ast))
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

#[proc_macro_derive(YaSerialize, attributes(yaserde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as syn::DeriveInput);

  common::validate(&ast)
    .and_then(|()| ser::expand_derive_serialize(&ast))
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

// Serialize & Deserialize a struct using its UpperHex implementation
#[proc_macro_derive(HexBinaryYaSerde)]
pub fn derive_hexbinary(input: TokenStream) -> TokenStream {
  let serde: TokenStream2 = hexbinary_serde(input.clone()).into();
  let yaserde: TokenStream2 = primitive_yaserde(input).into();

  quote!(
      use ::std::str::FromStr as _;
      #serde
      #yaserde
  )
  .into()
}

// Serialize & Deserialize a primitive newtype by generating a FromStr & Display implementation
#[proc_macro_derive(PrimitiveYaSerde)]
pub fn derive_primitive(input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as syn::DeriveInput);
  let serde = match primitive_serde(&ast) {
    Ok(serde) => serde,
    Err(error) => return error.into_compile_error().into(),
  };
  let yaserde: TokenStream2 = primitive_yaserde(quote!(#ast).into()).into();

  quote!(
      use ::std::str::FromStr as _;
      #serde
      #yaserde
  )
  .into()
}

// Serialize & Deserialize a type using its existing FromStr & Display implementation
#[proc_macro_derive(DefaultYaSerde)]
pub fn derive_default(input: TokenStream) -> TokenStream {
  primitive_yaserde(input)
}
