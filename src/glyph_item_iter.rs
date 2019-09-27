// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use GlyphItem;

//Note: This type not exported
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
    pub fn init_end(glyph_item: &GlyphItem, text: &str) -> Option<GlyphItemIter> {
        unsafe {
            let mut iter = GlyphItemIter::uninitialized();
            let ret = from_glib(pango_sys::pango_glyph_item_iter_init_end(
                iter.to_glib_none_mut().0,
                mut_override(glyph_item.to_glib_none().0),
                text.to_glib_none().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    pub fn init_start(glyph_item: &GlyphItem, text: &str) -> Option<GlyphItemIter> {
        unsafe {
            let mut iter = GlyphItemIter::uninitialized();
            let ret = from_glib(pango_sys::pango_glyph_item_iter_init_start(
                iter.to_glib_none_mut().0,
                mut_override(glyph_item.to_glib_none().0),
                text.to_glib_none().0,
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

    pub fn into_data(&self) -> GlyphItemIteratorData {
        GlyphItemIteratorData {
            start_glyph: self.0.start_glyph,
            end_glyph: self.0.end_glyph,
            start_index: self.0.start_index as usize,
            end_index: self.0.end_index as usize,
            start_char: self.0.start_char as usize,
            end_char: self.0.end_char as usize,
        }
    }
}

pub struct GlyphItemIteratorData {
    pub start_glyph: i32,
    pub start_index: usize,
    pub start_char: usize,

    pub end_glyph: i32,
    pub end_index: usize,
    pub end_char: usize,
}

pub struct GlyphItemIterator<'a> {
    item: &'a GlyphItem,
    text: &'a str,
    is_reverse: bool,
    iter: Option<GlyphItemIter>,
}

impl<'a> GlyphItemIterator<'a> {
    #[inline(always)]
    fn new_start(item: &'a GlyphItem, text: &'a str) -> GlyphItemIterator<'a> {
        GlyphItemIterator {
            item,
            text,
            is_reverse: false,
            iter: None,
        }
    }

    #[inline(always)]
    fn new_end(item: &'a GlyphItem, text: &'a str) -> GlyphItemIterator<'a> {
        GlyphItemIterator {
            item,
            text,
            is_reverse: true,
            iter: None,
        }
    }
}

impl GlyphItem {
    pub fn iter<'a>(
        &'a self,
        text: &'a str,
    ) -> impl DoubleEndedIterator<Item = GlyphItemIteratorData> + 'a {
        GlyphItemIterator::new_start(self, text)
    }

    pub fn riter<'a>(
        &'a self,
        text: &'a str,
    ) -> impl DoubleEndedIterator<Item = GlyphItemIteratorData> + 'a {
        GlyphItemIterator::new_end(self, text)
    }
}

impl<'a> Iterator for GlyphItemIterator<'a> {
    type Item = GlyphItemIteratorData;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.iter {
            if self.is_reverse {
                if iter.prev_cluster() {
                    Some(iter.into_data())
                } else {
                    None
                }
            } else {
                if iter.next_cluster() {
                    Some(iter.into_data())
                } else {
                    None
                }
            }
        } else {
            let iter = if self.is_reverse {
                GlyphItemIter::init_end(self.item, self.text)
            } else {
                GlyphItemIter::init_start(self.item, self.text)
            };
            if let Some(iter) = iter {
                let data = iter.into_data();
                self.iter = Some(iter);
                Some(data)
            } else {
                None
            }
        }
    }
}

impl<'a> DoubleEndedIterator for GlyphItemIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.iter {
            if self.is_reverse {
                if iter.next_cluster() {
                    Some(iter.into_data())
                } else {
                    None
                }
            } else {
                if iter.prev_cluster() {
                    Some(iter.into_data())
                } else {
                    None
                }
            }
        } else {
            let iter = if self.is_reverse {
                GlyphItemIter::init_start(self.item, self.text)
            } else {
                GlyphItemIter::init_end(self.item, self.text)
            };
            if let Some(iter) = iter {
                let data = iter.into_data();
                self.iter = Some(iter);
                Some(data)
            } else {
                None
            }
        }
    }
}
