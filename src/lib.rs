pub mod attr;

pub mod element;

pub mod error;

/// Internal helper macro for appending and attribute
/// into an XML element.
macro_rules! push_attr {
    ($var:expr, $writer:ident, $attr:literal <- String) => {
        $var.as_ref().inspect(|i| $writer.push_attribute(($attr, i.as_str())));
    };

    ($var:expr, $writer:ident, $attr:literal <- prim) => {
        $var.inspect(|i| $writer.push_attribute(($attr, i.to_string().as_str())));
    };

    ($var:expr, $writer:ident, $attr:literal <- ToString) => {
        $var.as_ref().inspect(|i| $writer.push_attribute(($attr, i.to_string().as_str())));
    };

    ($var:expr, $writer:ident, $attr:literal <- strings | $delim:literal) => {
        $var.as_ref().inspect(|s| $writer.push_attribute(($attr, s.join($delim).as_str())));
    };
}

pub(crate) use push_attr;
