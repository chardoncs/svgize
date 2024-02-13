#[cfg(feature = "cross_origin")]
pub mod referrer_policy;

#[cfg(feature = "attr_core")]
pub struct CoreAttr {
    pub id: Option<String>,
    pub lang: Option<String>,

    pub tab_index: Option<i32>,

    #[cfg(feature = "ns_xml")]
    pub xml_base: Option<String>,

    #[cfg(feature = "ns_xml")]
    pub xml_lang: Option<String>,

    #[cfg(all(feature = "ns_xml", feature = "deprecated"))]
    pub xml_space: Option<XmlSpace>,
}

#[cfg(all(feature = "ns_xml", feature = "deprecated"))]
pub enum XmlSpace {
    Default,
    Preserve,
}

#[cfg(feature = "attr_styling")]
pub struct StylingAttr {
    pub class: Option<String>,
    pub style: Option<String>,
}

#[cfg(feature = "attr_cond_proc")]
pub struct CondProcAttr {
    pub req_exts: Option<Vec<String>>,
    pub sys_lang: Option<Vec<String>>,
}

#[cfg(feature = "attr_event")]
pub struct EventAttr {

}
