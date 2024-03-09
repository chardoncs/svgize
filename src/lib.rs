pub mod attr;
pub mod constants;
pub mod element;
pub mod error;

/// Internal helper macro for appending an attribute
/// into an XML element.
macro_rules! push_attr {
    ($var:expr, $bs:ident, $attr:literal <- String) => {
        $var.as_ref().inspect(|i| $bs.push_attribute(($attr, i.as_str())));
    };

    ($var:expr, $bs:ident, $attr:literal <- prim$(itive)?) => {
        $var.inspect(|i| $bs.push_attribute(($attr, i.to_string().as_str())));
    };

    ($var:expr, $bs:ident, $attr:literal <- ToString) => {
        $var.as_ref().inspect(|i| $bs.push_attribute(($attr, i.to_string().as_str())));
    };

    ($var:expr, $bs:ident, $attr:literal <- strings | $delim:literal) => {
        $var.as_ref().inspect(|s| $bs.push_attribute(($attr, s.join($delim).as_str())));
    };

    ($var:expr, $bs:ident, $attr:literal <- primitives | $delim:literal) => {
        $var.as_ref().inspect(|s| $bs.push_attribute(($attr, s.iter().map(|item| item.to_string()).collect::<Vec<String>>().join($delim).as_str())));
    };

    (map: $var:expr, $bs:ident) => {
        if let Some(attr) = $var.as_ref() {
            crate::attr::WriteInAttr::write_in(attr, &mut $bs)?;
        }
    };
}

pub(crate) use push_attr;

/// Point in a 2D space
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

/// A mutable list of values
pub struct ValueList<T>
where
    T: ToString,
{
    inner: Vec<T>,
}

impl<T> ValueList<T>
where
    T: ToString,
{
    /// Instantiate a new value list
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    /// Append and move an additional value into the value list
    pub fn push(&mut self, value: T) -> &mut Self {
        self.inner.push(value);
        self
    }

    /// Convert the valut list into a string with item separated by
    /// a customized delimiter.
    pub fn into_string_with_delim(self, delim: &str) -> String {
        self.inner.into_iter().map(|item| item.to_string()).collect::<Vec<String>>().join(delim)
    }

    /// Convert the value list into a string with item separated by spaces.
    ///
    /// Same as `ValueList<T>::into_string_delim(self, " ")`.
    #[inline]
    pub fn into_string(self) -> String {
        self.into_string_with_delim(" ")
    }
}

impl<T> Into<String> for ValueList<T>
where
    T: ToString,
{
    #[inline]
    fn into(self) -> String {
        self.into_string()
    }
}

