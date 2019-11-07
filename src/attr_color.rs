use glib::translate::from_glib_none;
use glib::translate::from_glib_full;
use glib::translate::{FromGlibPtrFull, FromGlibPtrNone, Stash, ToGlibPtr};
use pango_sys;
use Attribute;
use Color;

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut pango_sys::PangoAttrColor> for &'a AttrColor {
    type Storage = &'a AttrColor;

    fn to_glib_none(&self) -> Stash<'a, *mut pango_sys::PangoAttrColor, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoAttrColor> for AttrColor {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoAttrColor) -> Self {
        assert!(!ptr.is_null());
        AttrColor(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut pango_sys::PangoAttrColor> for AttrColor {
    unsafe fn from_glib_full(ptr: *mut pango_sys::PangoAttrColor) -> Self {
        assert!(!ptr.is_null());
        AttrColor(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoAttrColor> for AttrColor {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoAttrColor) -> Self {
        assert!(!ptr.is_null());
        AttrColor(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const pango_sys::PangoAttrColor> for AttrColor {
    unsafe fn from_glib_full(ptr: *const pango_sys::PangoAttrColor) -> Self {
        assert!(!ptr.is_null());
        AttrColor(ptr as *mut _)
    }
}

#[derive(Debug)]
pub struct AttrColor(*mut pango_sys::PangoAttrColor);

impl AttrColor {
    pub fn get_color(&self) -> Color {
        unsafe {
            let color: *const pango_sys::PangoColor = &(*self.0).color;
            from_glib_none(color)
        }
    }
}

impl From<Attribute> for AttrColor {
    fn from(attribute: Attribute) -> Self {
        unsafe {
            let attr_color = std::mem::transmute::<*const pango_sys::PangoAttribute, *const pango_sys::PangoAttrColor>(attribute.to_glib_none().0);
            from_glib_full(attr_color)
        }
    }
}

impl PartialEq for AttrColor {
    fn eq(&self, other: &AttrColor) -> bool {
        self.0 == other.0
    }
}

impl Eq for AttrColor {}
