use quick_xml::events::BytesStart;

use crate::{attr::LazyAttrMap, element::{convert_into_xml, TagName}, push_attr};
#[cfg(feature = "attr-event")]
use crate::attr::LazyEventMap;

use super::{impl_children, impl_tag, impl_to_string, ElementNode, WriteXml};

pub struct Path {
    d: Option<String>,
    length: Option<f32>,
    
    attr: LazyAttrMap,

    #[cfg(feature = "attr-event")]
    ev_attr: LazyEventMap,
}

impl Path {
    pub fn new() -> Self {
        Self {
            d: None,
            length: None,
            attr: None,
            #[cfg(feature = "attr-event")]
            ev_attr: None,
        }
    }

    pub fn data(&self) -> Option<&str> {
        Some(self.d.as_ref()?.as_str())
    }

    pub fn set_data<T>(&mut self, data: &T)
    where
        T: ToString,
    {
        self.d = Some(data.to_string());
    }

    pub fn clear_data(&mut self) {
        self.d = None;
    }

    pub fn path_length(&self) -> Option<f32> {
        self.length
    }

    pub fn set_path_length(&mut self, len: f32) {
        self.length = Some(len);
    }

    pub fn clear_path_length(&mut self) {
        self.length = None;
    }
}

impl WriteXml for Path {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = self.tag_name();
            
        let mut bs = BytesStart::new(tag);

        push_attr!(self.d, bs, "d" <- String);

        convert_into_xml(writer, bs, None, tag)
    }
}

impl_tag!(Path, "path");
impl_children!(Path?);
impl_to_string!(Path);

impl ElementNode for Path {}
