use self::anchor::Anchor;

mod anchor;

pub trait ToTagName {
    fn to_tag_name(&self) -> String;
}

pub trait ElementNode: ToTagName + ToString {}

pub enum ElementKind {
    Anchor(Anchor),
    // TODO: Add more
}

pub enum ChildKind {
    String(String),
    Element(ElementKind),
}
