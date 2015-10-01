// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use libc::c_int;

use ::{Context, FontDescription, Rectangle};
use ffi::PangoRectangle;
use glib::translate::{Stash, FromGlibPtr, ToGlibPtr, from_glib_full, from_glib_none, from_glib};
use glib::{to_bool, to_gboolean};


/// Layout Objects — High-level layout driver objects
impl<'a> ToGlibPtr<'a, *mut ffi::PangoLayout> for &'a Layout {
    type Storage = &'a Layout;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoLayout, Self> {
        Stash(self.0, *self)
    }
}

impl FromGlibPtr<*mut ffi::PangoLayout> for Layout {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoLayout) -> Self {
        let tmp = ffi::pango_layout_copy(ptr);
        assert!(!tmp.is_null());
        Layout(tmp)
    }

    unsafe fn from_glib_full(ptr: *mut ffi::PangoLayout) -> Self {
        assert!(!ptr.is_null());
        Layout(ptr)
    }
}

pub struct Layout(*mut ffi::PangoLayout);


impl Layout {
    pub fn new(context: &Context) -> Option<Layout> {
        let tmp = unsafe { ffi::pango_layout_new(context.to_glib_none().0) };

        if tmp.is_null() {
            None
        } else {
            Some(Layout(tmp))
        }
    }

    pub fn copy(&self) -> Option<Layout> {
        let tmp = unsafe { ffi::pango_layout_copy(self.0) };

        if tmp.is_null() {
            None
        } else {
            Some(Layout(tmp))
        }
    }

    pub fn get_context(&self) -> Option<Context> {
        let tmp = unsafe { ffi::pango_layout_get_context(self.0) };

        if tmp.is_null
        () {
            None
        } else {
            Some(unsafe{Context::from_glib_full(tmp)})
        }
    }

    pub fn context_changed(&mut self) {
        unsafe { ffi::pango_layout_context_changed(self.0) };
    }

    pub fn get_serial(&self) -> u64 {
        (unsafe { ffi::pango_layout_get_serial(self.0) }) as u64
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe { ffi::pango_layout_set_text(self.0, text.to_glib_none().0, text.len() as  i32) };
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::pango_layout_get_text(self.0)) }
    }

    pub fn get_character_count(&self) -> i64 {
        (unsafe { ffi::pango_layout_get_character_count(self.0) }) as i64
    }

    pub fn set_markup(&mut self, text: &str) {
        unsafe { ffi::pango_layout_set_markup(self.0, text.to_glib_none().0, text.len() as i32) };
    }

    pub fn set_font_description(&mut self, desc: &FontDescription) {
        unsafe { ffi::pango_layout_set_font_description(self.0, desc.to_glib_none().0) };
    }

    // Not sure how to implement this as a *const FontDescription must be converted;
    // don't know which function / method to use
    // pub fn get_font_description(&self) -> Option<FontDescription> {
    //     let tmp = unsafe { ffi::pango_layout_get_font_description(self.0) };
    //
    //     if tmp.is_null() {
    //         None
    //     } else {
    //         Some(unsafe{FontDescription::from_glib_full(tmp)})
    //     }
    // }

    pub fn set_width(&mut self, width: i32) {
        unsafe { ffi::pango_layout_set_width(self.0, width as c_int ) };
    }

    pub fn get_width(&self) -> i32 {
        (unsafe { ffi::pango_layout_get_width(self.0) }) as i32
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { ffi::pango_layout_set_height(self.0, height as c_int ) };
    }

    pub fn get_height(&self) -> i32 {
        (unsafe { ffi::pango_layout_get_height(self.0) }) as i32
    }

    pub fn set_wrap(&mut self, wrap_mode: ffi::PangoWrapMode) {
        unsafe { ffi::pango_layout_set_wrap(self.0, wrap_mode ) };
    }

    pub fn get_wrap(&self) -> ffi::PangoWrapMode {
        unsafe { ffi::pango_layout_get_wrap(self.0) }
    }

    pub fn is_wrapped(&self) -> bool {
        unsafe { ffi::pango_layout_is_wrapped(self.0) != 0 }
    }

    pub fn set_ellipsize(&mut self, ellipsize_mode: ffi::PangoEllipsizeMode) {
        unsafe { ffi::pango_layout_set_ellipsize(self.0, ellipsize_mode ) };
    }

    pub fn get_ellipsize(&self) -> ffi::PangoEllipsizeMode {
        unsafe { ffi::pango_layout_get_ellipsize(self.0) }
    }

    pub fn is_ellipsized(&self) -> bool {
        unsafe { ffi::pango_layout_is_ellipsized(self.0) != 0 }
    }

    pub fn set_indent(&mut self, indent: i32) {
        unsafe { ffi::pango_layout_set_indent(self.0, indent as c_int ) };
    }

    pub fn get_indent(&self) -> i32 {
        (unsafe { ffi::pango_layout_get_indent(self.0) }) as i32
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        unsafe { ffi::pango_layout_set_spacing(self.0, spacing as c_int ) };
    }

    pub fn get_spacing(&self) -> i32 {
        (unsafe { ffi::pango_layout_get_spacing(self.0) }) as i32
    }

    pub fn set_justify(&mut self, justify: bool) {
        unsafe { ffi::pango_layout_set_spacing(self.0, to_gboolean(justify) ) };
    }

    pub fn get_justify(&self) -> bool {
        unsafe { ffi::pango_layout_get_spacing(self.0) != 0 }
    }

    pub fn set_alignment(&mut self, alignment: ffi::PangoAlignment) {
        unsafe { ffi::pango_layout_set_alignment(self.0, alignment ) };
    }

    pub fn get_alignment(&self) -> ffi::PangoAlignment {
        unsafe { ffi::pango_layout_get_alignment(self.0) }
    }

    pub fn set_single_paragraph_mode(&mut self, single_para: bool) {
        unsafe { ffi::pango_layout_set_single_paragraph_mode(self.0, to_gboolean(single_para) ) };
    }

    pub fn get_single_paragraph_mode(&mut self) -> bool {
        unsafe { ffi::pango_layout_get_single_paragraph_mode(self.0) != 0 }
    }

    pub fn get_unknown_glyphs_count(&self) -> i64 {
        (unsafe { ffi::pango_layout_get_unknown_glyphs_count(self.0) }) as i64
    }

    pub fn index_to_pos(&self, index: i64) -> PangoRectangle {
        let mut rect = PangoRectangle::new(0, 0, 1, 1);
        unsafe { ffi::pango_layout_index_to_pos(self.0, index as c_int, &mut rect) };
        rect
    }

    pub fn index_to_line_x(&self, index: i64, trailing: bool) -> (i32, i32) {
        let mut line: i32 = 0;
        let mut x_pos: i32 = 0;
        unsafe { ffi::pango_layout_index_to_line_x(self.0, index as c_int, to_gboolean(trailing),
            &mut line, &mut x_pos) };
        return (line, x_pos);
    }

    pub fn xy_to_index(&self, x: i64, y: i64) -> (bool, i32, bool) {
        let mut line: i32 = 0;
        let mut trailing: c_int = 0;
        let mut hit: c_int = unsafe {
            ffi::pango_layout_xy_to_index(self.0, x as c_int, y as c_int,
                &mut line, &mut trailing)
        };
        return (hit != 0, line, trailing != 0);
    }

    pub fn get_cursor_pos(&self, index: i64) -> (PangoRectangle, PangoRectangle) {
        let mut strong_pos = PangoRectangle::new(0, 0, 1, 1);
        let mut weak_pos = PangoRectangle::new(0, 0, 1, 1);
        unsafe { ffi::pango_layout_get_cursor_pos(self.0, index as c_int,
            &mut strong_pos, &mut weak_pos) };
        return (strong_pos, weak_pos);
    }

    pub fn move_cursor_visually(&self, strong: bool, old_index: i32, old_trailing: bool,
        direction: i32) -> (i32, bool) {
        let mut new_index: i32 = 0;
        let mut new_trailing: i32 = 0;
        unsafe { ffi::pango_layout_move_cursor_visually(self.0, strong as c_int,
            old_index as c_int, old_trailing as c_int, direction as c_int,
            &mut new_index, &mut new_trailing) };
        return (new_index, new_trailing != 0);
    }

    pub fn get_extents(&self) -> (PangoRectangle, PangoRectangle) {
        let mut ink_rect = PangoRectangle::new(0, 0, 1, 1);
        let mut logical_Rect = PangoRectangle::new(0, 0, 1, 1);
        unsafe { ffi::pango_layout_get_extents(self.0, &mut ink_rect, &mut logical_Rect) };
        return (ink_rect, logical_Rect);
    }

    pub fn get_pixel_extents(&self) -> (PangoRectangle, PangoRectangle) {
        let mut ink_rect = PangoRectangle::new(0, 0, 1, 1);
        let mut logical_Rect = PangoRectangle::new(0, 0, 1, 1);
        unsafe { ffi::pango_layout_get_pixel_extents(self.0, &mut ink_rect, &mut logical_Rect) };
        return (ink_rect, logical_Rect);
    }

    pub fn get_size(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe { ffi::pango_layout_get_size(self.0, &mut width, &mut height) };
        return (width, height);
    }

    pub fn get_pixel_size(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        unsafe { ffi::pango_layout_get_pixel_size(self.0, &mut width, &mut height) };
        return (width, height);
    }

    pub fn get_baseline(&self) -> i32 {
        unsafe { ffi::pango_layout_get_baseline(self.0) as i32 }
    }

    pub fn get_line_count(&self) -> i64 {
        unsafe { ffi::pango_layout_get_line_count(self.0) as i64 }
    }
}
