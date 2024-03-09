use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, element::{convert_into_xml, Children}, push_attr};

use super::{impl_accessor, impl_element, LazyChildList, TagName, WriteXml};

pub struct Polygon {
    points: Option<String>,
    path_length: Option<f32>,

    attr: LazyAttrMap,
    children: LazyChildList,
}

impl Default for Polygon {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            points: None,
            path_length: None,
            attr: None,
            children: None,
        }
    }
}

impl Polygon {
    #[inline]
    pub fn new<T>(points: &T) -> Self
    where
        T: ToString,
    {
        let mut p = Self::default();
        p.points = Some(points.to_string());
        p
    }

    impl_accessor!(string* -> points, set_points, "points");
    impl_accessor!(primitive -> path_length, set_path_length, f32, "pathLength");
}

impl_element!(Polygon, "polygon");
impl_attr_accessors!(Polygon);

impl WriteXml for Polygon {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = self.tag_name();

        let mut bs = BytesStart::new(tag);

        push_attr!(self.points, bs, "points" <- String);
        push_attr!(self.path_length, bs, "pathLength" <- prim);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, self.children(), tag)
    }
}
