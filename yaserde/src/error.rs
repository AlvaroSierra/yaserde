//! Error type returned by YaSerDe serialization and deserialization.

use std::fmt;

/// Result type returned by YaSerDe serialization and deserialization.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error returned by YaSerDe serialization and deserialization.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  /// The XML input is malformed, as reported by the XML reader.
  Reader(xml::reader::Error),
  /// The XML output could not be written.
  Writer(xml::writer::Error),
  /// The XML does not match the expected structure, or a value could not be converted.
  Message(String),
  /// An error located in a field, named by its XML name.
  Field {
    /// XML name of the field
    name: String,
    /// The error in that field
    source: Box<Error>,
  },
}

impl Error {
  /// Create an error from any message, for instance in a custom `YaDeserialize` implementation.
  pub fn custom<T: fmt::Display>(message: T) -> Self {
    Error::Message(message.to_string())
  }

  /// Wrap this error to locate it in the field with the given XML name.
  pub fn in_field<T: Into<String>>(self, name: T) -> Self {
    Error::Field {
      name: name.into(),
      source: Box::new(self),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Reader(error) => write!(f, "{}", error),
      Error::Writer(error) => write!(f, "{}", error),
      Error::Message(message) => f.write_str(message),
      Error::Field { name, source } => write!(f, "{}: {}", name, source),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::Reader(error) => Some(error),
      Error::Writer(error) => Some(error),
      Error::Message(_) => None,
      Error::Field { source, .. } => Some(source.as_ref()),
    }
  }
}

impl From<xml::reader::Error> for Error {
  fn from(error: xml::reader::Error) -> Self {
    Error::Reader(error)
  }
}

impl From<xml::writer::Error> for Error {
  fn from(error: xml::writer::Error) -> Self {
    Error::Writer(error)
  }
}

impl From<String> for Error {
  fn from(message: String) -> Self {
    Error::Message(message)
  }
}

impl From<&str> for Error {
  fn from(message: &str) -> Self {
    Error::Message(message.to_string())
  }
}

#[test]
fn display() {
  let error = Error::custom("invalid digit found in string").in_field("item");
  assert_eq!(error.to_string(), "item: invalid digit found in string");
  assert!(std::error::Error::source(&error).is_some());
}
