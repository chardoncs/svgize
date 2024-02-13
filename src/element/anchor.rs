#[cfg(feature = "cross_origin")]
use crate::attr::referrer_policy::ReferrerPolicy;
#[cfg(feature = "attr_core")]
use crate::attr::CoreAttr;
#[cfg(feature = "attr_styling")]
use crate::attr::StylingAttr;

use super::ChildKind;

pub struct Anchor {
    pub download: Option<String>,

    pub href: Option<String>,

    pub href_lang: Option<String>,

    #[cfg(feature = "exp")]
    pub ping: Option<Vec<String>>,

    #[cfg(feature = "cross_origin")]
    pub referrer_policy: Option<ReferrerPolicy>,

    #[cfg(feature = "attr_core")]
    pub core: CoreAttr,

    #[cfg(feature = "attr_styling")]
    pub styling: StylingAttr,

    pub children: Option<Vec<ChildKind>>,
}
