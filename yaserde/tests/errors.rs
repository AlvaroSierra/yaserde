#[macro_use]
extern crate yaserde_derive;

use yaserde::de::from_str;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn de_no_content() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(rename = "book")]
  pub struct Book {
    author: String,
    title: String,
  }

  let content = "";
  let loaded: Result<Book, String> = from_str(content);
  assert_eq!(
    loaded,
    Err("Unexpected end of stream: no root element found".to_owned())
  );
}

#[test]
fn de_wrong_end_balise() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(rename = "book")]
  pub struct Book {
    author: String,
    title: String,
  }

  let content = "<book><author>Antoine de Saint-Exupéry<title>Little prince</title></book>";
  let loaded: Result<Book, String> = from_str(content);
  assert_eq!(
    loaded,
    Err("author: Expected end tag </author>, found start tag <title>".to_owned())
  );
}

#[test]
fn de_invalid_value() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(rename = "data")]
  pub struct Data {
    required: Option<u32>,
  }

  let loaded: Result<Data, String> = from_str("<data><required>abc</required></data>");
  assert_eq!(
    loaded,
    Err("required: invalid digit found in string".to_owned())
  );
}

#[test]
fn de_invalid_text_content() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(rename = "data")]
  pub struct Data {
    #[yaserde(text = true)]
    value: u32,
  }

  let loaded: Result<Data, String> = from_str("<data>abc</data>");
  assert_eq!(loaded, Err("invalid digit found in string".to_owned()));
}

#[test]
fn de_invalid_enum_value() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug, Default)]
  #[yaserde(rename = "data")]
  pub enum Data {
    #[default]
    Empty,
    Value(u32),
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct Base {
    data: Data,
  }

  let loaded: Result<Base, String> = from_str("<base><data><Value>abc</Value></data></base>");
  assert_eq!(loaded, Err("invalid digit found in string".to_owned()));
}
