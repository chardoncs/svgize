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

    (map: $var:expr, $writer:ident) => {
        if let Some(attr) = $var.as_ref() {
            crate::attr::WriteInAttr::write_in(attr, &mut $writer)?;
        }
    };
}

pub(crate) use push_attr;

macro_rules! stringifiable_enum {
    {[$type_name:ident] $($entry:ident, $attr:literal;)*} => {
        pub enum $type_name {
            $(
                $entry,
            )*
        }

        impl ToString for $type_name {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Self::$entry => $attr,
                    )*
                }.to_string()
            }
        }
    };
}

pub(crate) use stringifiable_enum;

pub struct Point(f32, f32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}
