// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

use libc::c_int;

use ::FontDescription;
use glib::translate::{Stash, FromGlibPtr, ToGlibPtr, from_glib_full, from_glib_none, from_glib};


impl<'a> ToGlibPtr<'a, *mut ffi::PangoContext> for &'a Context {
    type Storage = &'a Context;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoContext, Self> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtr<*mut ffi::PangoContext> for Context {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoContext) -> Self {
        // let tmp = ffi::pango_context_copy(ptr);
        // assert!(!tmp.is_null());
        // Context(tmp)
        assert!(!ptr.is_null());
        Context(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut ffi::PangoContext) -> Self {
        assert!(!ptr.is_null());
        Context(ptr)
    }
}

pub struct Context(*mut ffi::PangoContext);




impl Context {
    pub fn new() -> Option<Context> {
        let tmp = unsafe { ffi::pango_context_new() };

        if tmp.is_null() {
            None
        } else {
            Some(Context(tmp))
        }
    }

    // pub fn copy(&self) -> Option<Context> {
    //     let tmp = unsafe { ffi::pango_context_copy(self.0) };
    //
    //     if tmp.is_null() {
    //         None
    //     } else {
    //         Some(Context(tmp))
    //     }
    // }

    pub fn changed(&mut self) {
        unsafe { ffi::pango_context_changed(self.0) };
    }

    pub fn get_serial(&self) -> u64 {
        let serial = unsafe { ffi::pango_context_get_serial(self.0) };
        serial as u64
    }

    // NOT IMPLEMENTED:
    // pub fn get_font_map(&self) -> ...
    // pub fn set_font_map(&mut self, ...) -> ...

    pub fn get_font_description(&self) -> Option<FontDescription> {
        let tmp = unsafe { ffi::pango_context_get_font_description(self.0) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe{FontDescription::from_glib_full(tmp)})
        }
    }

    pub fn set_font_description(&mut self, desc: &FontDescription) {
        unsafe { ffi::pango_context_set_font_description(self.0, desc.to_glib_none().0) };
    }

    // NOT IMPLEMENTED:
    // pub fn get_language(&self) -> ...
    // pub fn set_language(&mut self, ...) -> ...

    pub fn get_base_dir(&self) -> ffi::PangoDirection {
        unsafe { ffi::pango_context_get_base_dir(self.0) }
    }

    pub fn set_base_dir(&mut self, direction: ffi::PangoDirection) {
        unsafe { ffi::pango_context_set_base_dir(self.0, direction) };
    }

    pub fn get_base_gravity(&self) -> ffi::PangoGravity {
        unsafe { ffi::pango_context_get_base_gravity(self.0) }
    }

    pub fn set_base_gravity(&mut self, gravity: ffi::PangoGravity) {
        unsafe { ffi::pango_context_set_base_gravity(self.0, gravity) };
    }

    pub fn get_gravity(&self) -> ffi::PangoGravity {
        unsafe { ffi::pango_context_get_gravity(self.0) }
    }

    pub fn get_gravity_hint(&self) -> ffi::PangoGravityHint {
        unsafe { ffi::pango_context_get_gravity_hint(self.0) }
    }

    pub fn set_gravity_hint(&mut self, gravity: ffi::PangoGravityHint) {
        unsafe { ffi::pango_context_set_gravity_hint(self.0, gravity) };
    }

    pub fn get_matrix(&self) -> &ffi::PangoMatrix {
        unsafe { &*ffi::pango_context_get_matrix(self.0) }
    }

    pub fn set_matrix(&mut self, matrix: &ffi::PangoMatrix) {
        unsafe { ffi::pango_context_set_matrix(self.0, matrix) };
    }

    // NOT IMPLEMENTED:
    // pub fn load_font(&mut self) -> ...
    // pub fn load_fontset(&mut self, ...) -> ...
    // pub fn get_metrics(&self, ...) -> ...
    // pub fn list_families(&self, ...) -> ...
}
