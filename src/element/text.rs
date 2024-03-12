use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, length_adjust::LengthAdjust, LazyAttrMap}, element::{convert_into_xml, Children}, push_attr};

use super::{impl_accessor, impl_element, ChildList, TagName, WriteXml};

/// Text element (`<text>`)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text).
pub struct Text {
    x: Option<String>,
    y: Option<String>,

    dx: Option<String>,
    dy: Option<String>,

    rotate: Option<Vec<f32>>,

    length_adjust: Option<LengthAdjust>,

    text_length: Option<String>,

    attr: LazyAttrMap,

    children: Option<ChildList>,
}

impl Default for Text {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            dx: None,
            dy: None,
            rotate: None,
            length_adjust: None,
            text_length: None,
            attr: None,
            children: None,
        }
    }
}

impl Text {
    #[inline]
    pub fn new<TX, TY>(x: &TX, y: &TY) -> Self
    where
        TX: ToString,
        TY: ToString,
    {
        let mut t = Self::default();
        
        t.set_x(Some(x));
        t.set_y(Some(y));

        t
    }

    impl_accessor!(string* -> x, set_x, "x");
    impl_accessor!(string* -> y, set_y, "y");
    impl_accessor!(string* -> dx, set_dx, "dx");
    impl_accessor!(string* -> dy, set_dy, "dy");
    impl_accessor!(list:primitive -> rotate, rotate_mut, "rotate", f32);
    impl_accessor!(ref:move_setter -> length_adjust, set_length_adjust, "lengthAdjust", LengthAdjust);
    impl_accessor!(string* -> text_length, set_text_length, "textLength");
}

impl_element!(Text, "text");
impl_attr_accessors!(Text);

impl WriteXml for Text {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = Self::tag_name();

        let mut bs = BytesStart::new(tag);

        push_attr!(self.x, bs, "x" <- String);
        push_attr!(self.y, bs, "y" <- String);
        push_attr!(self.dx, bs, "dx" <- String);
        push_attr!(self.dy, bs, "dy" <- String);
        push_attr!(self.rotate, bs, "rotate" <- primitives | " ");
        push_attr!(self.length_adjust, bs, "lengthAdjust" <- ToString);
        push_attr!(self.text_length, bs, "textLength" <- String);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, self.children(), tag)
    }
}
