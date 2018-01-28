// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Device;
use DeviceType;
use Display;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DeviceManager(Object<ffi::GdkDeviceManager>);

    match fn {
        get_type => || ffi::gdk_device_manager_get_type(),
    }
}

pub trait DeviceManagerExt {
    #[cfg_attr(feature = "v3_20", deprecated)]
    fn get_client_pointer(&self) -> Option<Device>;

    fn get_display(&self) -> Option<Display>;

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn list_devices(&self, type_: DeviceType) -> Vec<Device>;

    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_device_changed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceManager> + IsA<glib::object::Object>> DeviceManagerExt for O {
    fn get_client_pointer(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_device_manager_get_client_pointer(self.to_glib_none().0))
        }
    }

    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_device_manager_get_display(self.to_glib_none().0))
        }
    }

    fn list_devices(&self, type_: DeviceType) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_device_manager_list_devices(self.to_glib_none().0, type_.to_glib()))
        }
    }

    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "device-added",
                transmute(device_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_device_changed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "device-changed",
                transmute(device_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "device-removed",
                transmute(device_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn device_added_trampoline<P>(this: *mut ffi::GdkDeviceManager, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<DeviceManager> {
    callback_guard!();
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&DeviceManager::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(device))
}

unsafe extern "C" fn device_changed_trampoline<P>(this: *mut ffi::GdkDeviceManager, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<DeviceManager> {
    callback_guard!();
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&DeviceManager::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(device))
}

unsafe extern "C" fn device_removed_trampoline<P>(this: *mut ffi::GdkDeviceManager, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<DeviceManager> {
    callback_guard!();
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&DeviceManager::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(device))
}

unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkDeviceManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceManager> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceManager::from_glib_borrow(this).downcast_unchecked())
}
