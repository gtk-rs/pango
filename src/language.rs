// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Script;
use ffi;
use glib::translate::*;

pub struct Language(*mut ffi::PangoLanguage);

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::PangoLanguage> for &'a Language {
    type Storage = &'a Language;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoLanguage, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::PangoLanguage> for Language {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::PangoLanguage, Self> {
        StashMut(self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *mut ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *const ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *const ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr as *mut _)
    }
}

impl Default for Language {
    fn default() -> Language {
        unsafe { from_glib_full(ffi::pango_language_get_default()) }
    }
}

impl Language {
    pub fn from_string(language: &str) -> Language {
        unsafe { from_glib_full(ffi::pango_language_from_string(language.to_glib_none().0)) }
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::pango_language_to_string(self.to_glib_none().0)) }
    }

    pub fn matches(&self, range_list: &str) -> bool {
        unsafe { ffi::pango_language_matches(self.to_glib_none().0,
                                             range_list.to_glib_none().0).to_bool() }
    }

    pub fn includes_script(&self, script: Script) -> bool {
        unsafe { ffi::pango_language_includes_script(self.to_glib_none().0, script.to_glib()).to_bool() }
    }

    pub fn get_scripts(&self) -> Vec<Script> {
        let mut num_scripts = 0;
        let mut ret = Vec::new();

        unsafe {
            let scripts: *const ffi::PangoScript = ffi::pango_language_get_scripts(self.to_glib_none().0,
                                                                                   &mut num_scripts);
            if num_scripts > 0 {
                for x in 0..num_scripts {
                    ret.push(from_glib(*(scripts.offset(x as isize) as *const ffi::PangoScript)));
                }
            }
            ret
        }
    }

    pub fn get_sample_string(&self) -> String {
        unsafe { from_glib_full(ffi::pango_language_get_sample_string(self.to_glib_none().0)) }
    }
}
