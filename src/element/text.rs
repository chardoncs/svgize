use crate::attr::{length_adjust::LengthAdjust, LazyAttrMap};

use super::ChildList;

/// Text element (`<text>`)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text).
pub struct Text {
    x: Option<String>,
    y: Option<String>,

    dx: Option<String>,
    dy: Option<String>,

    rotate: Option<Vec<f32>>,

    length_adjust: LengthAdjust,

    text_length: Option<String>,

    attr: LazyAttrMap,

    children: Option<ChildList>,
}

// TODO
