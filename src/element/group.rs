use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, element::{convert_into_xml, Children}, push_attr};

use super::{impl_element, ChildKind, LazyChildList, TagName, WriteXml};

/// SVG group element (<g>)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g).
pub struct Group {
    attr: LazyAttrMap,
    children: LazyChildList,
}

impl Default for Group {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            attr: None,
            children: None,
        }
    }
}

impl Group {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_children<T>(iter: T) -> Self
    where
        T: Iterator<Item = ChildKind>,
    {
        let mut g = Self::new();
        g.children = Some(iter.collect::<Vec<ChildKind>>());
        g
    }
}

impl_element!(Group, "g");
impl_attr_accessors!(Group);

impl WriteXml for Group {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = Self::tag_name();

        let mut bs = BytesStart::new(tag);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, self.children(), tag)
    }
}
