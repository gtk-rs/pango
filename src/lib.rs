// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate pango_sys as ffi;
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate libc;

mod auto;
pub use auto::*;
pub use auto::functions::*;

pub use ffi::PANGO_SCALE as SCALE;

pub mod prelude;

pub mod attr_class;
pub use attr_class::AttrClass;
pub mod attribute;
pub mod font_description;
pub mod item;
pub use item::Item;
pub mod language;
pub use language::Language;
pub mod matrix;
pub mod rectangle;
pub use rectangle::Rectangle;
