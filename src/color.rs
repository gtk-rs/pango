// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use Color;

impl Color {
    pub fn red(&self) -> u16 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).red
        }
    }

    pub fn green(&self) -> u16 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).green
        }
    }

    pub fn blue(&self) -> u16 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).blue
        }
    }
}
