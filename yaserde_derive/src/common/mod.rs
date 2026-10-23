mod attribute;
mod field;

pub use attribute::YaSerdeAttribute;
pub use field::{Field, YaSerdeField};

/// Check the `#[yaserde(...)]` attributes and the field types of the input before expansion,
/// so that errors are reported at the offending code instead of as a macro panic.
pub fn validate(ast: &syn::DeriveInput) -> syn::Result<()> {
  let mut errors: Option<syn::Error> = None;
  let mut push = |result: syn::Result<()>| {
    if let Err(error) = result {
      match errors {
        Some(ref mut errors) => errors.combine(error),
        None => errors = Some(error),
      }
    }
  };
  let check_attributes = |attrs: &[syn::Attribute]| YaSerdeAttribute::parse(attrs).map(|_| ());
  let check_field = |field: &syn::Field| {
    check_attributes(&field.attrs).and_then(|()| Field::try_from(&field.ty).map(|_| ()))
  };

  push(check_attributes(&ast.attrs));

  match &ast.data {
    syn::Data::Struct(data_struct) => {
      if let syn::Fields::Unnamed(fields) = &data_struct.fields {
        push(Err(syn::Error::new_spanned(
          fields,
          "YaSerDe does not support tuple structs, use named fields \
           or derive `PrimitiveYaSerde`/`DefaultYaSerde` for a newtype",
        )));
      }
      data_struct
        .fields
        .iter()
        .for_each(|field| push(check_field(field)));
    }
    syn::Data::Enum(data_enum) => {
      for variant in &data_enum.variants {
        push(check_attributes(&variant.attrs));
        variant
          .fields
          .iter()
          .for_each(|field| push(check_field(field)));
      }
    }
    syn::Data::Union(_) => {
      return Err(syn::Error::new_spanned(
        &ast.ident,
        "YaSerDe does not support unions",
      ));
    }
  }

  errors.map_or(Ok(()), Err)
}
