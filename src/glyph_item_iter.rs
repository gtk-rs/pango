// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use GlyphItem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GlyphItemIter(Boxed<pango_sys::PangoGlyphItemIter>);

    match fn {
        copy => |ptr| pango_sys::pango_glyph_item_iter_copy(mut_override(ptr)),
        free => |ptr| pango_sys::pango_glyph_item_iter_free(ptr),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || pango_sys::pango_glyph_item_iter_get_type(),
    }
}

impl GlyphItemIter {
    pub fn init_end(glyph_item: &mut GlyphItem) -> Option<GlyphItemIter> {
        unsafe {
            let mut iter = GlyphItemIter::uninitialized();
            let ret = from_glib(pango_sys::pango_glyph_item_iter_init_end(
                iter.to_glib_none_mut().0,
                glyph_item.to_glib_none_mut().0,
                //Text seems ignored and item's used
                "".to_glib_none().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    pub fn init_start(glyph_item: &mut GlyphItem) -> Option<GlyphItemIter> {
        unsafe {
            let mut iter = GlyphItemIter::uninitialized();
            let ret = from_glib(pango_sys::pango_glyph_item_iter_init_start(
                iter.to_glib_none_mut().0,
                glyph_item.to_glib_none_mut().0,
                "".to_glib_none().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    pub fn next_cluster(&mut self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_glyph_item_iter_next_cluster(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn prev_cluster(&mut self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_glyph_item_iter_prev_cluster(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn start_index(&self) -> i32 {
        self.0.start_index
    }

    pub fn start_char(&self) -> i32 {
        self.0.start_char
    }

    pub fn start_glyph(&self) -> i32 {
        self.0.start_glyph
    }

    pub fn end_index(&self) -> i32 {
        self.0.end_index
    }

    pub fn end_char(&self) -> i32 {
        self.0.end_char
    }

    pub fn end_glyph(&self) -> i32 {
        self.0.end_glyph
    }
}
