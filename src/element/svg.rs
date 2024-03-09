use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, constants::SVG_NAMESPACE, element::convert_into_xml, push_attr};

use super::{impl_accessor, impl_element, ChildList, TagName, WriteXml};

/// SVG container element (`<svg>`)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg).
pub struct Svg {
    view_box: Option<String>,

    width: Option<String>,
    height: Option<String>,

    x: Option<String>,
    y: Option<String>,

    preserve_aspect_ratio: Option<String>,

    attr: LazyAttrMap,
    children: Option<ChildList>,
}

impl Default for Svg {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            view_box: None,
            width: None,
            height: None,
            x: None,
            y: None,
            preserve_aspect_ratio: None,
            attr: None,
            children: None,
        }
    }
}

impl Svg {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize SVG element with `viewBox` attribute.
    pub fn with_view_box<T>(view_box: &T) -> Self
    where
        T: ToString,
    {
        let mut svg = Self::new();
        
        svg.set_view_box(Some(view_box));

        svg
    }

    impl_accessor!(string* -> view_box, set_view_box, "viewBox");
    impl_accessor!(string* -> x, set_x, "x");
    impl_accessor!(string* -> y, set_y, "y");
    impl_accessor!(string* -> width, set_width, "width");
    impl_accessor!(string* -> height, set_height, "height");
    impl_accessor!(string* -> preserve_aspect_ratio, set_preserve_aspect_ratio, "preserveAspectRatio");
}

impl_element!(Svg, "svg");
impl_attr_accessors!(Svg);

impl WriteXml for Svg {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = self.tag_name();

        let mut bs = BytesStart::new(tag);

        push_attr!(self.view_box, bs, "viewBox" <- String);
        push_attr!(self.x, bs, "x" <- String);
        push_attr!(self.y, bs, "y" <- String);
        push_attr!(self.width, bs, "width" <- String);
        push_attr!(self.height, bs, "height" <- String);
        push_attr!(self.preserve_aspect_ratio, bs, "preserveAspectRatio" <- String);

        push_attr!(map: self.attr, bs);

        // Add XML namespace for SVG
        bs.push_attribute(("xmlns", SVG_NAMESPACE));

        convert_into_xml(writer, bs, self.children.as_ref(), tag)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_svg_element() {
        assert_eq!(Svg::new().to_string(), r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#);
    }
}
