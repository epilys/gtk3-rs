// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkUtil")]
    pub struct Util(Object<ffi::AtkUtil, ffi::AtkUtilClass>);

    match fn {
        type_ => || ffi::atk_util_get_type(),
    }
}

impl Util {
    pub const NONE: Option<&'static Util> = None;
}

impl fmt::Display for Util {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Util")
    }
}
