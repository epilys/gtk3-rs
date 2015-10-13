// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use Container;
use Orientable;
use Orientation;
use Widget;
use gdk;

pub type Paned = Object<ffi::GtkPaned>;

unsafe impl Upcast<Widget> for Paned { }
unsafe impl Upcast<Container> for Paned { }
unsafe impl Upcast<Buildable> for Paned { }
unsafe impl Upcast<Orientable> for Paned { }

impl Paned {
    pub fn new(orientation: Orientation) -> Paned {
        unsafe {
            Widget::from_glib_none(ffi::gtk_paned_new(orientation)).downcast_unchecked()
        }
    }

    pub fn add1<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_paned_add1(self.to_glib_none().0, child.upcast().to_glib_none().0);
        }
    }

    pub fn add2<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_paned_add2(self.to_glib_none().0, child.upcast().to_glib_none().0);
        }
    }

    pub fn get_child1(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_child1(self.to_glib_none().0))
        }
    }

    pub fn get_child2(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_child2(self.to_glib_none().0))
        }
    }

    pub fn get_handle_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_handle_window(self.to_glib_none().0))
        }
    }

    pub fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_paned_get_position(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_16)]
    pub fn get_wide_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paned_get_wide_handle(self.to_glib_none().0))
        }
    }

    pub fn pack1<T: Upcast<Widget>>(&self, child: &T, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack1(self.to_glib_none().0, child.upcast().to_glib_none().0, resize.to_glib(), shrink.to_glib());
        }
    }

    pub fn pack2<T: Upcast<Widget>>(&self, child: &T, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack2(self.to_glib_none().0, child.upcast().to_glib_none().0, resize.to_glib(), shrink.to_glib());
        }
    }

    pub fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.to_glib_none().0, position);
        }
    }

    #[cfg(gtk_3_16)]
    pub fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.to_glib_none().0, wide.to_glib());
        }
    }

}

impl types::StaticType for Paned {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_paned_get_type()) }
    }
}
