use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, element::{convert_into_xml, Children}, push_attr};

use super::{impl_accessor, impl_element, ChildList, TagName, WriteXml};

/// Circle element (`<circle>`)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/circle).
pub struct Circle {
    cx: Option<String>,
    cy: Option<String>,
    radius: Option<String>,
    
    attr: LazyAttrMap,

    children: Option<ChildList>,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            cx: None,
            cy: None,
            radius: None,
            attr: None,
            children: None,
        }
    }
}

impl Circle {
    /// Create a new circle
    ///
    /// ## Parameters
    ///
    /// - center: For `cx` and `cy` attributes
    /// - radius: For `r` attributes
    pub fn new<TCX, TCY, TR>(center: (&TCX, &TCY), radius: &TR) -> Self
    where
        TCX: ToString,
        TCY: ToString,
        TR: ToString,
    {
        let mut c = Circle::default();

        c.set_cx(Some(center.0));
        c.set_cy(Some(center.1));

        c.set_radius(Some(radius));

        c
    }

    impl_accessor!(string* -> cx, set_cx, "cx");
    impl_accessor!(string* -> cy, set_cy, "cy");
    impl_accessor!(string* -> radius, set_radius, "r");
}

impl_element!(Circle, "circle");
impl_attr_accessors!(Circle);

impl WriteXml for Circle {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = self.tag_name();

        let mut bs = BytesStart::new(tag);

        push_attr!(self.cx, bs, "cx" <- String);
        push_attr!(self.cy, bs, "cy" <- String);
        push_attr!(self.radius, bs, "r" <- String);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, self.children(), tag)
    }
}
