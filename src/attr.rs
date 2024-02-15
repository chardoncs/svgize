use quick_xml::events::BytesStart;

use crate::{error::Error, push_attr};

#[cfg(feature = "crossorigin")]
pub mod referrer_policy;

pub trait WriteInAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error>;
}

/// SVG Core Attributes
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Core).
#[cfg(feature = "attr-core")]
pub struct CoreAttr {
    /// `id`: A unique identifier.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/id).
    pub id: Option<String>,

    /// `lang`: Language of the element.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lang).
    pub lang: Option<String>,

    /// `tabindex`: Tab index responsible to the focusing order.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/tabindex).
    pub tab_index: Option<i32>,

    /// `xml:base`: Specifies a base IRI other than the base IRI of the document.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:base).
    #[cfg(feature = "ns-xml")]
    pub xml_base: Option<String>,

    /// `xml:lang`: Language attribute available in all XML dialects.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:lang).
    #[cfg(feature = "ns-xml")]
    pub xml_lang: Option<String>,

    /// `xml:space`: **Deprecated in XML standard**. Specifying and handling white space characters.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:space).
    #[cfg(all(feature = "ns-xml", feature = "deprecated"))]
    pub xml_space: Option<XmlSpace>,
}

#[cfg(feature = "attr-core")]
impl Default for CoreAttr {
    fn default() -> Self {
        Self {
            id: None,
            lang: None,
            tab_index: None,
            #[cfg(feature = "ns-xml")]
            xml_base: None,
            #[cfg(feature = "ns-xml")]
            xml_lang: None,
            #[cfg(all(feature = "ns-xml", feature = "deprecated"))]
            xml_space: None,
        }
    }
}

#[cfg(feature = "attr-core")]
impl WriteInAttr for CoreAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        push_attr!(self.id, bs, "id" <- String);
        push_attr!(self.lang, bs, "lang" <- String);
        push_attr!(self.tab_index, bs, "tabindex" <- prim);

        #[cfg(feature = "ns-xml")]
        push_attr!(self.xml_base, bs, "xml:base" <- String);
        #[cfg(feature = "ns-xml")]
        push_attr!(self.xml_lang, bs, "xml:lang" <- String);
            
        #[cfg(all(feature = "ns-xml", feature = "deprecated"))]
        push_attr!(self.xml_space, bs, "xml:space" <- ToString);

        Ok(())
    }
}

/// XML space enumeration
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:space#usage_notes).
#[cfg(all(feature = "ns-xml", feature = "deprecated"))]
pub enum XmlSpace {
    /// `default`
    Default,
    /// `preserve`
    Preserve,
}

#[cfg(all(feauter = "ns-xml", feature = "deprecated"))]
impl ToString for XmlSpace {
    fn to_string(&self) -> String {
        match self {
            XmlSpace::Default => "default",
            XmlSpace::Preserve => "preserve",
        }.to_string()
    }
}

/// SVG Styling Attributes
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Styling).
#[cfg(feature = "attr-styling")]
pub struct StylingAttr {
    /// `class`: Assign one or multiple class names to current element.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/class).
    pub class: Option<String>,
    /// `style`: Style information of current element.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/style).
    pub style: Option<String>,
}

#[cfg(feature = "attr-styling")]
impl Default for StylingAttr {
    fn default() -> Self {
        Self {
            class: None,
            style: None,
        }
    }
}

#[cfg(feature = "attr-styling")]
impl WriteInAttr for StylingAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        push_attr!(self.class, bs, "class" <- String);
        push_attr!(self.style, bs, "style" <- String);

        Ok(())
    }
}

/// SVG Conditional Processing Attributes
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Conditional_Processing).
#[cfg(feature = "attr-cond_proc")]
pub struct CondProcAttr {
    /// `requiredExtensions`: List all the browser specific capabilities that 
    /// must be supported by the browser to be allowed to render the associated element.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Conditional_Processing#attributes).
    pub req_exts: Option<Vec<String>>,
    /// `systemLanguage`: Indicates which language the user must have chosen to render the associated element.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/systemLanguage).
    pub sys_lang: Option<Vec<String>>,

    /// `requiredFeatures`: **Deprecated in SVG standard**.
    /// List all the features, [as defined in the SVG 1.1 specification](https://www.w3.org/TR/SVG11/feature.html),
    /// that must be supported by the browser to be allowed to render the associated element.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/requiredFeatures).
    #[cfg(feature = "deprecated")]
    pub req_features: Option<Vec<String>>,
}

#[cfg(feature = "attr-cond_proc")]
impl Default for CondProcAttr {
    fn default() -> Self {
        Self {
            req_exts: None,
            sys_lang: None,
            #[cfg(feature = "deprecated")]
            req_features: None,
        }
    }
}

#[cfg(feature = "attr-cond_proc")]
impl WriteInAttr for CondProcAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        push_attr!(self.req_exts, bs, "requiredExtensions" <- strings | " ");
        push_attr!(self.sys_lang, bs, "systemLanguage" <- strings | ",");

        #[cfg(feature = "deprecated")]
        push_attr!(self.req_features, bs, "requiredFeatures" <- strings | " ");

        Ok(())
    }
}

/// SVG Event Attributes
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Events).
#[cfg(feature = "attr-event")]
pub struct EventAttr {

}

/// SVG Presentation Attributes
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Presentation).
#[cfg(feature = "attr-presentation")]
pub struct PresentationAttr {
}
