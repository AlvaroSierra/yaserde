mod attribute;
mod field;

pub use attribute::YaSerdeAttribute;
pub use field::{Field, YaSerdeField};

/// Check every `#[yaserde(...)]` attribute of the input before expansion,
/// so that errors are reported at the offending attribute instead of as a macro panic.
pub fn validate(ast: &syn::DeriveInput) -> syn::Result<()> {
  let mut errors: Option<syn::Error> = None;
  let mut check = |attrs: &[syn::Attribute]| {
    if let Err(error) = YaSerdeAttribute::parse(attrs) {
      match errors {
        Some(ref mut errors) => errors.combine(error),
        None => errors = Some(error),
      }
    }
  };

  check(&ast.attrs);

  match &ast.data {
    syn::Data::Struct(data_struct) => {
      data_struct
        .fields
        .iter()
        .for_each(|field| check(&field.attrs));
    }
    syn::Data::Enum(data_enum) => {
      for variant in &data_enum.variants {
        check(&variant.attrs);
        variant.fields.iter().for_each(|field| check(&field.attrs));
      }
    }
    syn::Data::Union(_) => {
      return Err(syn::Error::new_spanned(
        &ast.ident,
        "YaSerDe does not support unions",
      ))
    }
  }

  errors.map_or(Ok(()), Err)
}
