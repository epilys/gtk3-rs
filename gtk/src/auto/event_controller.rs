// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{PropagationPhase, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkEventController")]
    pub struct EventController(Object<ffi::GtkEventController, ffi::GtkEventControllerClass>);

    match fn {
        type_ => || ffi::gtk_event_controller_get_type(),
    }
}

impl EventController {
    pub const NONE: Option<&'static EventController> = None;
}

pub trait EventControllerExt: 'static {
    #[doc(alias = "gtk_event_controller_get_propagation_phase")]
    #[doc(alias = "get_propagation_phase")]
    fn propagation_phase(&self) -> PropagationPhase;

    #[doc(alias = "gtk_event_controller_get_widget")]
    #[doc(alias = "get_widget")]
    fn widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_event_controller_handle_event")]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[doc(alias = "gtk_event_controller_reset")]
    fn reset(&self);

    #[doc(alias = "gtk_event_controller_set_propagation_phase")]
    fn set_propagation_phase(&self, phase: PropagationPhase);

    #[doc(alias = "propagation-phase")]
    fn connect_propagation_phase_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EventController>> EventControllerExt for O {
    fn propagation_phase(&self) -> PropagationPhase {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_propagation_phase(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_handle_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
            ))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_event_controller_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_propagation_phase(&self, phase: PropagationPhase) {
        unsafe {
            ffi::gtk_event_controller_set_propagation_phase(
                self.as_ref().to_glib_none().0,
                phase.into_glib(),
            );
        }
    }

    fn connect_propagation_phase_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_propagation_phase_trampoline<
            P: IsA<EventController>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkEventController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(EventController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::propagation-phase\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_propagation_phase_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventController")
    }
}
