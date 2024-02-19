#[derive(Debug)]
pub enum Error {
    NoChildrenExpected,
    NoOptionFound,
    XmlWriterError(quick_xml::Error),
    Utf8ParseError(std::str::Utf8Error),
}
