#[cfg(feature = "sparse_attr")]
use std::collections::HashMap;

use quick_xml::events::BytesStart;

use crate::{error::Error, push_attr};

#[cfg(feature = "crossorigin")]
pub mod referrer_policy;

pub trait WriteInAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error>;
}

#[cfg(feature = "sparse_attr")]
pub trait AttrKey {
    fn attr_key(&self) -> String;
}

#[cfg(feature = "sparse_attr")]
impl<K, V> WriteInAttr for HashMap<K, V>
where
    K: AttrKey,
    V: ToString,
{
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        for (key, val) in self.iter() {
            bs.push_attribute((key.attr_key().as_str(), val.to_string().as_str()));
        }

        Ok(())
    }
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
}

#[cfg(feature = "attr-core")]
impl Default for CoreAttr {
    fn default() -> Self {
        Self {
            id: None,
            lang: None,
            tab_index: None,
        }
    }
}

#[cfg(feature = "attr-core")]
impl WriteInAttr for CoreAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        push_attr!(self.id, bs, "id" <- String);
        push_attr!(self.lang, bs, "lang" <- String);
        push_attr!(self.tab_index, bs, "tabindex" <- prim);

        Ok(())
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
}

#[cfg(feature = "attr-cond_proc")]
impl Default for CondProcAttr {
    fn default() -> Self {
        Self {
            req_exts: None,
            sys_lang: None,
        }
    }
}

#[cfg(feature = "attr-cond_proc")]
impl WriteInAttr for CondProcAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        push_attr!(self.req_exts, bs, "requiredExtensions" <- strings | " ");
        push_attr!(self.sys_lang, bs, "systemLanguage" <- strings | ",");

        Ok(())
    }
}

#[cfg(feature = "sparse_attr")]
macro_rules! def_sparse_attr {
    {[$type_name:ident] $($entry:ident, $attr:literal;)*} => {
        /// SVG Event Attributes
        ///
        /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Events).
        pub enum $type_name {
            $(
                $entry,
            )*
        }

        impl AttrKey for $type_name {
            fn attr_key(&self) -> String {
                match self {
                    $(
                        Self::$entry => $attr,
                    )*
                }.to_string()
            }
        }
    };
}

#[cfg(feature = "attr-event")]
def_sparse_attr! {
    [AnimationEventAttr]
    OnBegin, "onbegin";
    OnEnd, "onend";
    OnRepeat, "onrepeat";
}

#[cfg(feature = "attr-event")]
def_sparse_attr! {
    [DocEventAttr]
    OnAbort, "onabort";
    OnError, "onerror";
    OnResize, "onresize";
    OnScroll, "onscroll";
    OnUnload, "onunload";
}

#[cfg(feature = "attr-event")]
def_sparse_attr! {
    [DocElementEventAttr]
    OnCopy, "oncopy";
    OnCut, "oncut";
    OnPaste, "onpaste";
}

#[cfg(feature = "attr-event")]
def_sparse_attr! {
    [GlobalEventAttr]
    OnCancel, "oncancel";
    OnCanPlay, "oncanplay";
    OnCanPlayThrough, "oncanplaythrough";
    OnChange, "onchange";
    OnClick, "onclick";
    OnClose, "onclose";
    OnCueChange, "oncuechange";
    OnDoubleClick, "ondblclick";
    OnDrag, "ondrag";
    OnDragEnd, "ondragend";
    OnDragEnter, "ondragenter";
    OnDragLeave, "ondragleave";
    OnDragOver, "ondragover";
    OnDragStart, "ondragstart";
    OnDrop, "ondrop";
    OnDurationChange, "ondurationchange";
    OnEmptied, "onemptied";
    OnEnded, "onended";
    OnError, "onerror";
    OnFocus, "onfocus";
    OnInput, "oninput";
    OnInvalid, "oninvalid";
    OnKeyDown, "onkeydown";
    OnKeyPress, "onkeypress";
    OnKeyUp, "onkeyup";
    OnLoad, "onload";
    OnLoadedData, "onloadeddata";
    OnLoadedMetadata, "onloadedmetadata";
    OnLoadStart, "onloadstart";
    OnMouseDown, "onmousedown";
    OnMouseEnter, "onmouseenter";
    OnMouseLeave, "onmouseleave";
    OnMouseMove, "onmousemove";
    OnMouseOut, "onmouseout";
    OnMouseOver, "onmouseover";
    OnMouseUp, "onmouseup";
    OnMouseWheel, "onmousewheel";
    OnPause, "onpause";
    OnPlay, "onplay";
    OnPlaying, "onplaying";
    OnProgress, "onprogress";
    OnRateChange, "onratechange";
    OnReset, "onreset";
    OnResize, "onresize";
    OnScroll, "onscroll";
    OnSought, "onseeked"; // I don't think "seeked" is correct English... :/
    OnSeeking, "onseeking";
    OnSelect, "onselect";
    OnShow, "onshow";
    OnStalled, "onstalled";
    OnSubmit, "onsubmit";
    OnSuspend, "onsuspend";
    OnTimeUpdate, "ontimeupdate";
    OnToggle, "ontoggle";
    OnVolumeChange, "onvolumechange";
    OnWaiting, "onwaiting";
}

#[cfg(feature = "attr-event")]
def_sparse_attr! {
    [GraphicalEventAttr]
    OnActivate, "onactivate";
    OnFocusIn, "onfocusin";
    OnFocusOut, "onfocusout";
}

/// SVG Presentation Attributes
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Presentation).
#[cfg(feature = "attr-presentation")]
pub struct PresentationAttr {
}
