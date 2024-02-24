use std::io::Cursor;

use quick_xml::{events::{BytesEnd, BytesStart, BytesText, Event}, Writer};

use crate::error::Error;

pub use self::path::Path;
pub use self::svg::Svg;
pub use self::rect::Rect;

pub mod path;
pub mod svg;
pub mod rect;

/// Instance having a tag name.
pub trait TagName {
    /// Access the tag name of current instance.
    fn tag_name(&self) -> &str;
}

/// Instance having child nodes.
pub trait Children {
    /// Access read-only reference child list.
    fn children(&self) -> Option<&Vec<ChildKind>>;

    /// Access mutable reference child list.
    fn children_mut(&mut self) -> Result<&mut Vec<ChildKind>, Error>;
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
        impl crate::element::TagName for $struct_name {
            #[inline]
            fn tag_name(&self) -> &str {
                $tag
            }
        }
    };
}

pub(crate) use impl_tag;

/// Internal helper macro for implementing children trait.
macro_rules! impl_children {
    ($struct_name:tt) => {
        impl crate::element::Children for $struct_name {
            fn children(&self) -> Option<&crate::element::ChildList> {
                self.children.as_ref()
            }

            fn children_mut(&mut self) -> Result<&mut Vec<crate::element::ChildKind>, crate::error::Error> {
                if self.children.is_none() {
                    self.children = Some(Vec::new());
                }

                Ok(self.children.as_mut().unwrap()) // There must be something
            }
        }
    };

    ($struct_name:tt ?) => {
        impl crate::element::Children for $struct_name {
            fn children(&self) -> Option<&Vec<crate::element::ChildKind>> {
                None
            }

            fn children_mut(&mut self) -> Result<&mut Vec<crate::element::ChildKind>, crate::error::Error> {
                Err(crate::error::Error::NoChildrenExpected)
            }
        }
    };
}

pub(crate) use impl_children;

pub type ChildList = Vec<ChildKind>;

pub type LazyChildList = Option<ChildList>;

/// Instances that might be converted to string.
pub trait TryToString {
    /// Try to convert current instance to string.
    fn try_to_string(&self) -> Result<String, crate::error::Error>;
}

macro_rules! impl_to_string {
    ($struct_name:tt) => {
        impl crate::element::TryToString for $struct_name {
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
                crate::element::TryToString::try_to_string(self).unwrap()
            }
        }
    };
}

pub(crate) use impl_to_string;

pub(crate) fn convert_into_xml(writer: &mut Writer<Cursor<Vec<u8>>>, bs: BytesStart, children: Option<&ChildList>, tag: &str) -> Result<(), Error> {
    if let Some(children) = children.as_ref() {
        writer.write_event(Event::Start(bs))
            .or_else(|err| Err(Error::XmlWriterError(err)))?;

        for child in children.iter() {
            match child {
                ChildKind::String(ref content) => {
                    writer.write_event(Event::Text(BytesText::new(content.as_str())))
                        .or_else(|err| Err(Error::XmlWriterError(err)))?;
                }
                ChildKind::Element(ref el) => {
                    el.write_xml(writer)?;
                }
            }
        }

        writer.write_event(Event::End(BytesEnd::new(tag)))
            .or_else(|err| Err(Error::XmlWriterError(err)))?;
    } else {
        writer.write_event(Event::Empty(bs))
            .or_else(|err| Err(Error::XmlWriterError(err)))?;
    }

    Ok(())
}

macro_rules! def_element_kind {
    ($($type_name:tt),*) => {
        pub enum ElementKind {
            $($type_name($type_name),)*
        }

        impl WriteXml for ElementKind {
            fn write_xml(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), Error> {
                Ok(match self {
                    $(
                        ElementKind::$type_name(inner) => inner.write_xml(writer)?,
                    )*
                })
            }
        }

        impl Children for ElementKind {
            fn children(&self) -> Option<&Vec<ChildKind>> {
                match self {
                    $(
                        ElementKind::$type_name(inner) => inner.children(),
                    )*
                }
            }

            fn children_mut(&mut self) -> Result<&mut Vec<ChildKind>, Error> {
                match self {
                    $(
                        ElementKind::$type_name(inner) => inner.children_mut(),
                    )*
                }
            }

        }
    };
}

/// Child kind enumeration
pub enum ChildKind {
    /// Raw string
    String(String),
    /// Other elements
    Element(ElementKind),
}

macro_rules! impl_element {
    ($name:ident, $tag:literal) => {
        crate::element::impl_tag!($name, $tag);
        crate::element::impl_children!($name);
        crate::element::impl_to_string!($name);

        impl crate::element::ElementNode for $name {}
    };

    ($name:ident?, $tag:literal) => {
        crate::element::impl_tag!($name, $tag);
        crate::element::impl_children!($name?);
        crate::element::impl_to_string!(Path);

        impl crate::element::ElementNode for $name {}
    };
}

pub(crate) use impl_element;

macro_rules! impl_accessor {
    (string -> $name:ident, $setter:ident, $attr:literal) => {
        #[doc = "Get `"]
        #[doc = $attr]
        #[doc = "`\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]
        pub fn $name(&self) -> Option<&str> {
            Some(self.$name.as_ref()?.as_str())
        }

        #[doc = "Set `"]
        #[doc = $attr]
        #[doc = "`\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]

        pub fn $setter(&mut self, value: Option<&str>) -> &mut Self {
            self.$name = value.and_then(|value| Some(value.to_string()));
            self
        }
    };
    
    (string* -> $name:ident, $setter:ident, $attr:literal) => {
        #[doc = "Get `"]
        #[doc = $attr]
        #[doc = "`\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]
        pub fn $name(&self) -> Option<&str> {
            Some(self.$name.as_ref()?.as_str())
        }

        #[doc = "Set `"]
        #[doc = $attr]
        #[doc = "`\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]

        pub fn $setter<T>(&mut self, value: Option<&T>) -> &mut Self
        where
            T: ToString,
        {
            self.$name = value.and_then(|value| Some(value.to_string()));
            self
        }
    };

    (string -> $name:ident, $setter:ident, $attr:literal) => {
        #[doc = "Get `"]
        #[doc = $attr]
        #[doc = "`\n\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]
        pub fn $name(&self) -> Option<&str> {
            Some(self.$name.as_ref()?.as_str())
        }

        #[doc = "Set `"]
        #[doc = $attr]
        #[doc = "`\n\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]

        pub fn $setter<T>(&mut self, value: Option<&T>) -> &mut Self
        where
            T: ToString,
        {
            self.$name = value.and_then(|value| Some(value.to_string()));
            self
        }
    };
    (primitive -> $name:ident, $setter:ident, $type:ty, $attr:literal) => {
        #[doc = "Get `"]
        #[doc = $attr]
        #[doc = "`\n\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]
        pub fn $name(&self) -> Option<$type> {
            self.$name
        }

        #[doc = "Set `"]
        #[doc = $attr]
        #[doc = "`\n\n"]
        #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
        #[doc = $attr]
        #[doc = ")."]

        pub fn $setter<T>(&mut self, value: Option<$type>) -> &mut Self {
            self.$name = value;
            self
        }
    };
}

pub(crate) use impl_accessor;

def_element_kind!(Path);
impl_to_string!(ElementKind);

