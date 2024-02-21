use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, element::convert_into_xml, push_attr};

use super::{impl_element, ChildList, TagName, WriteXml};

pub struct Rect {
    x: Option<String>,
    y: Option<String>,

    width: Option<String>,
    height: Option<String>,

    rx: Option<String>,
    ry: Option<String>,

    path_length: Option<f32>,

    attr: LazyAttrMap,

    children: Option<ChildList>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            width: None,
            height: None,
            rx: None,
            ry: None,
            path_length: None,
            attr: None,
            children: None,
        }
    }
}

impl Rect {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn x(&self) -> Option<&str> {
        Some(self.x.as_ref()?.as_str())
    }

    pub fn set_x<T>(&mut self, value: Option<&T>) -> &mut Self
    where
        T: ToString,
    {
        self.x = value.and_then(|value| Some(value.to_string()));
        self
    }
}

impl WriteXml for Rect {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = self.tag_name();

        let mut bs = BytesStart::new("tag");

        push_attr!(self.x, bs, "x" <- String);
        push_attr!(self.y, bs, "y" <- String);
        push_attr!(self.width, bs, "width" <- String);
        push_attr!(self.height, bs, "height" <- String);
        push_attr!(self.rx, bs, "rx" <- String);
        push_attr!(self.ry, bs, "ry" <- String);
        push_attr!(self.path_length, bs, "pathLength" <- prim);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, self.children.as_ref(), tag)
    }
}

impl_element!(Rect, "rect");
impl_attr_accessors!(Rect);
