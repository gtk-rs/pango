// This file was generated by gir (9e3f4cc) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct EngineShape(Object<ffi::PangoEngineShape>);

    match fn {
        get_type => || ffi::pango_engine_shape_get_type(),
    }
}

impl EngineShape {}
