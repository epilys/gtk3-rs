// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::button::ButtonImpl;
use crate::ToggleButton;

pub trait ToggleButtonImpl: ToggleButtonImplExt + ButtonImpl {
    fn toggled(&self) {
        self.parent_toggled()
    }
}

pub trait ToggleButtonImplExt: ObjectSubclass {
    fn parent_toggled(&self);
}

impl<T: ToggleButtonImpl> ToggleButtonImplExt for T {
    fn parent_toggled(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkToggleButtonClass;
            if let Some(f) = (*parent_class).toggled {
                f(self
                    .obj()
                    .unsafe_cast_ref::<ToggleButton>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: ToggleButtonImpl> IsSubclassable<T> for ToggleButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.toggled = Some(button_toggled::<T>);
    }
}

unsafe extern "C" fn button_toggled<T: ToggleButtonImpl>(ptr: *mut ffi::GtkToggleButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.toggled()
}
