use std::io::Cursor;

use quick_xml::Writer;

use crate::error::Error;

use self::anchor::Anchor;

mod anchor;

/// Instance having a tag name.
pub trait TagName {
    /// Access the tag name of current instance.
    fn tag_name(&self) -> String;
}

/// Instance having child nodes.
pub trait Children {
    /// Access read-only reference child list.
    fn children(&self) -> Option<&Vec<ChildKind>>;

    /// Access mutable reference child list.
    fn children_mut(&mut self) -> &mut Vec<ChildKind>;
}

/// Instance can be written as XML through the `quick_xml` writer.
pub trait WriteXml {
    /// Convert current instance into XML by `quick_xml`'s writer.
    fn write_xml(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), Error>;
}

/// Element node trait.
pub trait ElementNode: TagName + WriteXml + ToString + TryToString + Children {}

/// Internal helper macro for implementing tag name trait.
macro_rules! impl_tag {
    ($struct_name:tt, $tag:literal) => {
        impl TagName for $struct_name {
            fn tag_name(&self) -> String {
                $tag.to_string()
            }
        }
    };
}

pub(crate) use impl_tag;

/// Internal helper macro for implementing children trait.
macro_rules! impl_children {
    ($struct_name:tt) => {
        impl Children for $struct_name {
            fn children(&self) -> Option<&Vec<ChildKind>> {
                self.children.as_ref()
            }

            fn children_mut(&mut self) -> &mut Vec<ChildKind> {
                if self.children.is_none() {
                    self.children = Some(Vec::new());
                }

                self.children.as_mut().unwrap() // There must be something
            }
        }
    };
}

pub(crate) use impl_children;

/// Instances that might be converted to string.
pub trait TryToString {
    /// Try to convert current instance to string.
    fn try_to_string(&self) -> Result<String, crate::error::Error>;
}

macro_rules! impl_to_string {
    ($struct_name:tt) => {
        impl TryToString for $struct_name {
            fn try_to_string(&self) -> Result<String, crate::error::Error> {
                let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
                self.write_xml(&mut writer)?;

                let out = writer.into_inner().into_inner();
                Ok(std::str::from_utf8(&out)
                   .or_else(|err| Err(crate::error::Error::Utf8ParseError(err)))?
                   .to_string())
            }
        }

        impl ToString for $struct_name {
            fn to_string(&self) -> String {
                self.try_to_string().unwrap()
            }
        }
    };
}

pub(crate) use impl_to_string;

pub enum ElementKind {
    Anchor(Anchor),
    // TODO: Add more
}

pub enum ChildKind {
    String(String),
    Element(ElementKind),
}
