use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, element::{convert_into_xml, Children}, push_attr};

use super::{impl_accessor, impl_element, ChildList, TagName, WriteXml};

/// Rectangle element (`<rect>`)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/rect).
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
    #[inline]
    #[must_use]
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
    /// Create an SVG rectangle with `x`, `y`, `width`, and `height`.
    pub fn new<TX, TY, TW, TH>(x: &TX, y: &TY, width: &TW, height: &TH) -> Self
    where
        TX: ToString,
        TY: ToString,
        TW: ToString,
        TH: ToString,
    {
        let mut rect = Self::default();

        rect.set_x(Some(x))
            .set_y(Some(y))
            .set_width(Some(width))
            .set_height(Some(height));

        rect
    }

    impl_accessor!(string* -> x, set_x, "x");
    impl_accessor!(string* -> y, set_y, "y");
    impl_accessor!(string* -> width, set_width, "width");
    impl_accessor!(string* -> height, set_height, "height");
    impl_accessor!(string* -> rx, set_rx, "rx");
    impl_accessor!(string* -> ry, set_ry, "ry");
    impl_accessor!(primitive -> path_length, set_path_length, f32, "pathLength");
}

impl WriteXml for Rect {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = Self::tag_name();

        let mut bs = BytesStart::new(tag);

        push_attr!(self.x, bs, "x" <- String);
        push_attr!(self.y, bs, "y" <- String);
        push_attr!(self.width, bs, "width" <- String);
        push_attr!(self.height, bs, "height" <- String);
        push_attr!(self.rx, bs, "rx" <- String);
        push_attr!(self.ry, bs, "ry" <- String);
        push_attr!(self.path_length, bs, "pathLength" <- prim);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, self.children(), tag)
    }
}

impl_element!(Rect, "rect");
impl_attr_accessors!(Rect);
