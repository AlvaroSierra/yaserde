use std::{io::Read, io::Write};

use crate::{Error, Result, de, ser};

pub fn serialize_primitives<S, W: Write>(
  self_bypass: &S,
  default_name: &str,
  writer: &mut ser::Serializer<W>,
  serialize_function: impl FnOnce(&S) -> String,
) -> Result<()> {
  let name = writer
    .get_start_event_name()
    .unwrap_or_else(|| default_name.to_string());

  if !writer.skip_start_end() {
    writer.write(xml::writer::XmlEvent::start_element(name.as_str()))?;
  }

  writer.write(xml::writer::XmlEvent::characters(
    serialize_function(self_bypass).as_str(),
  ))?;

  if !writer.skip_start_end() {
    writer.write(xml::writer::XmlEvent::end_element())?;
  }

  Ok(())
}

pub fn deserialize_primitives<S, R: Read>(
  reader: &mut de::Deserializer<R>,
  deserialize_function: impl FnOnce(&str) -> Result<S>,
) -> Result<S> {
  if let xml::reader::XmlEvent::StartElement { .. } = reader.peek()? {
    reader.next_event()?;
  } else {
    return Err(Error::custom("Start element not found"));
  }

  if let xml::reader::XmlEvent::Characters(text) = reader.peek()? {
    deserialize_function(text)
  } else {
    deserialize_function("")
  }
}
