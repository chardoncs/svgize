use std::{collections::HashMap, ops::Index};

use quick_xml::events::BytesStart;

use crate::error::Error;

#[cfg(feature = "crossorigin")]
pub mod referrer_policy;

pub trait WriteInAttr {
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error>;
}

pub trait AttrKey {
    fn attr_key(&self) -> &str;
}

impl<K> WriteInAttr for HashMap<K, String>
where
    K: AttrKey,
{
    fn write_in(&self, bs: &mut BytesStart) -> Result<(), Error> {
        for (key, val) in self.iter() {
            bs.push_attribute((key.attr_key(), val.as_str()));
        }

        Ok(())
    }
}

macro_rules! def_sparse_attr {
    {
        [$type_name:ident]
        $(
            {
                $(
                    $(#[$entry_macro:meta])*
                    $entry:ident, $attr:literal;
                )*
            }
        )?
        $(
            #[$proc_macro:meta] {
                $(
                    $(#[$entry_macro_c:meta])*
                    $entry_c:ident, $attr_c:literal;
                )*
            }
        )*
    } => {
        /// SVG Attributes
        ///
        /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute).
        #[derive(PartialEq, Eq, Hash)]
        pub enum $type_name {
            $($(
                $(#[$entry_macro])*
                #[doc = "The `"]
                #[doc = $attr]
                #[doc = "` attribute.\n\n"]
                #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
                #[doc = $attr]
                #[doc = ")."]
                $entry,
            )*)?
            $(
                $(
                    #[$proc_macro]
                    $(#[$entry_macro_c])*
                    #[doc = "The `"]
                    #[doc = $attr_c]
                    #[doc = "` attribute.\n\n"]
                    #[doc = "See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/"]
                    #[doc = $attr_c]
                    #[doc = ")."]
                    $entry_c,
                )*
            )*
        }

        impl AttrKey for $type_name {
            fn attr_key(&self) -> &str {
                match self {
                    $($(
                        $(#[$entry_macro])*
                        Self::$entry => $attr,
                    )*)?
                    $($(
                        #[$proc_macro]
                        $(#[$entry_macro_c])*
                        Self::$entry_c => $attr_c,
                    )*)*
                }
            }
        }

        impl ToString for $type_name {
            fn to_string(&self) -> String {
                self.attr_key().to_string()
            }
        }
    };
}

def_sparse_attr! {
    [Attr]
    #[cfg(feature = "attr-core")] {
        Id, "id";
        Lang, "lang";
        TabIndex, "tabindex";
    }
    #[cfg(feature = "attr-styling")] {
        ClassName, "class";
        Style, "style";
    }
    #[cfg(feature = "attr-cond_proc")] {
        ReqExt, "requiredExtensions";
        SysLang, "systemLanguage";
    }
    #[cfg(feature = "attr-presentation")] {
        AlignmentBaseline, "alignment-baseline";
        BaselineShift, "baseline-shift";
        ClipPath, "clip-path";
        ClipRule, "clip-rule";
        Color, "color";
        ColorInterpolation, "color-interpolation";
        ColorInterpolationFilters, "color-interpolation-filters";
        ColorRendering, "color-rendering";
        Cursor, "cursor";
        Direction, "direction";
        Display, "display";
        DominantBaseline, "dominant-baseline";
        Fill, "fill";
        FillOpacity, "fill-opacity";
        FillRule, "fill-rule";
        Filter, "filter";
        FloodColor, "flood-color";
        FloodOpacity, "flood-opacity";
        FontFamily, "font-family";
        FontSize, "font-size";
        FontSizeAdjust, "font-size-adjust";
        FontStretch, "font-stretch";
        FontStyle, "font-style";
        FontVariant, "font-variant";
        FontWeight, "font-weight";
        ImageRendering, "image-rendering";
        LetterSpacing, "letter-spacing";
        LightingColor, "lighting-color";
        MarkerEnd, "marker-end";
        MarkerMiddle, "marker-mid";
        MarkerStart, "marker-start";
        Mask, "mask";
        Opacity, "opacity";
        Overflow, "overflow";
        PointerEvents, "pointer-events";
        ShapeRendering, "shape-rendering";
        SolidColor, "solid-color";
        SolidOpacity, "solid-opacity";
        StopColor, "stop-color";
        StopOpacity, "stop-opacity";
        Stroke, "stroke";
        StrokeDashArray, "stroke-dasharray";
        StrokeDashOffset, "stroke-dashoffset";
        StrokeLineCap, "stroke-linecap";
        StrokeLineJoin, "stroke-linejoin";
        StrokeMiterLimit, "stroke-miterlimit";
        StrokeOpacity, "stroke-opacity";
        StrokeWidth, "stroke-width";
        TextAnchor, "text-anchor";
        TextDecoration, "text-decoration";
        TextRendering, "text-rendering";
        Transform, "transform";
        UnicodeBidi, "unicode-bidi";
        VectorEffect, "vector-effect";
        Visibility, "visibility";
        WordSpacing, "word-spacing";
        WritingMode, "writing-mode";
    }
    #[cfg(feature = "attr-event")] {
        OnBegin, "onbegin";
        OnEnd, "onend";
        OnRepeat, "onrepeat";
        OnAbort, "onabort";
        OnScroll, "onscroll";
        OnUnload, "onunload";
        OnCopy, "oncopy";
        OnCut, "oncut";
        OnPaste, "onpaste";
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
        OnActivate, "onactivate";
        OnFocusIn, "onfocusin";
        OnFocusOut, "onfocusout";
    }
}

pub type AttrMap = HashMap<Attr, String>;

pub type LazyAttrMap = Option<AttrMap>;

pub trait AccessAttr {
    fn attr(&self, attr: &Attr) -> Option<&str>;

    fn set_attr(&mut self, attr: Attr, value: &str);

    fn pop_attr(&mut self, attr: &Attr) -> Option<String>;
}

macro_rules! impl_attr_accessors {
    ($name:ident) => {
        impl crate::attr::AccessAttr for $name {
            fn attr(&self, attr: &crate::attr::Attr) -> Option<&str> {
                Some(self.attr.as_ref()?.get(attr)?.as_str())
            }

            fn set_attr(&mut self, attr: crate::attr::Attr, value: &str) {
                if self.attr.is_none() {
                    self.attr = Some(std::collections::HashMap::new());
                }

                self.attr.as_mut().unwrap()
                    .entry(attr)
                    .and_modify(|cur| *cur = value.to_string())
                    .or_insert(value.to_string());
            }

            fn pop_attr(&mut self, attr: &crate::attr::Attr) -> Option<String> {
                self.attr.as_mut()?.remove(attr)
            }
        }
    };
}

pub(crate) use impl_attr_accessors;
