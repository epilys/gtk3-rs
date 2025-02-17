// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, Bin, Buildable, Container, PopoverConstraint, PositionType, ResizeMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkPopover")]
    pub struct Popover(Object<ffi::GtkPopover, ffi::GtkPopoverClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    pub const NONE: Option<&'static Popover> = None;

    #[doc(alias = "gtk_popover_new")]
    pub fn new(relative_to: Option<&impl IsA<Widget>>) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new(
                relative_to.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_popover_new_from_model")]
    #[doc(alias = "new_from_model")]
    pub fn from_model(
        relative_to: Option<&impl IsA<Widget>>,
        model: &impl IsA<gio::MenuModel>,
    ) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new_from_model(
                relative_to.map(|p| p.as_ref()).to_glib_none().0,
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Popover`] objects.
    ///
    /// This method returns an instance of [`PopoverBuilder`](crate::builders::PopoverBuilder) which can be used to create [`Popover`] objects.
    pub fn builder() -> PopoverBuilder {
        PopoverBuilder::new()
    }
}

impl Default for Popover {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Popover`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PopoverBuilder {
    builder: glib::object::ObjectBuilder<'static, Popover>,
}

impl PopoverBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn constrain_to(self, constrain_to: PopoverConstraint) -> Self {
        Self {
            builder: self.builder.property("constrain-to", constrain_to),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn pointing_to(self, pointing_to: &gdk::Rectangle) -> Self {
        Self {
            builder: self.builder.property("pointing-to", pointing_to),
        }
    }

    pub fn position(self, position: PositionType) -> Self {
        Self {
            builder: self.builder.property("position", position),
        }
    }

    pub fn relative_to(self, relative_to: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("relative-to", relative_to.clone().upcast()),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Popover`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Popover {
        self.builder.build()
    }
}

pub trait PopoverExt: 'static {
    #[doc(alias = "gtk_popover_bind_model")]
    fn bind_model(&self, model: Option<&impl IsA<gio::MenuModel>>, action_namespace: Option<&str>);

    #[doc(alias = "gtk_popover_get_constrain_to")]
    #[doc(alias = "get_constrain_to")]
    fn constrain_to(&self) -> PopoverConstraint;

    #[doc(alias = "gtk_popover_get_default_widget")]
    #[doc(alias = "get_default_widget")]
    fn default_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_popover_get_modal")]
    #[doc(alias = "get_modal")]
    fn is_modal(&self) -> bool;

    #[doc(alias = "gtk_popover_get_pointing_to")]
    #[doc(alias = "get_pointing_to")]
    fn pointing_to(&self) -> Option<gdk::Rectangle>;

    #[doc(alias = "gtk_popover_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> PositionType;

    #[doc(alias = "gtk_popover_get_relative_to")]
    #[doc(alias = "get_relative_to")]
    fn relative_to(&self) -> Option<Widget>;

    #[doc(alias = "gtk_popover_popdown")]
    fn popdown(&self);

    #[doc(alias = "gtk_popover_popup")]
    fn popup(&self);

    #[doc(alias = "gtk_popover_set_constrain_to")]
    fn set_constrain_to(&self, constraint: PopoverConstraint);

    #[doc(alias = "gtk_popover_set_default_widget")]
    fn set_default_widget(&self, widget: Option<&impl IsA<Widget>>);

    #[doc(alias = "gtk_popover_set_modal")]
    fn set_modal(&self, modal: bool);

    #[doc(alias = "gtk_popover_set_pointing_to")]
    fn set_pointing_to(&self, rect: &gdk::Rectangle);

    #[doc(alias = "gtk_popover_set_position")]
    fn set_position(&self, position: PositionType);

    #[doc(alias = "gtk_popover_set_relative_to")]
    fn set_relative_to(&self, relative_to: Option<&impl IsA<Widget>>);

    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "constrain-to")]
    fn connect_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "modal")]
    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pointing-to")]
    fn connect_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "position")]
    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "relative-to")]
    fn connect_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Popover>> PopoverExt for O {
    fn bind_model(&self, model: Option<&impl IsA<gio::MenuModel>>, action_namespace: Option<&str>) {
        unsafe {
            ffi::gtk_popover_bind_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                action_namespace.to_glib_none().0,
            );
        }
    }

    fn constrain_to(&self) -> PopoverConstraint {
        unsafe {
            from_glib(ffi::gtk_popover_get_constrain_to(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_default_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_popover_get_modal(self.as_ref().to_glib_none().0)) }
    }

    fn pointing_to(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_popover_get_pointing_to(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    fn position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_popover_get_position(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_relative_to(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn popdown(&self) {
        unsafe {
            ffi::gtk_popover_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            ffi::gtk_popover_popup(self.as_ref().to_glib_none().0);
        }
    }

    fn set_constrain_to(&self, constraint: PopoverConstraint) {
        unsafe {
            ffi::gtk_popover_set_constrain_to(
                self.as_ref().to_glib_none().0,
                constraint.into_glib(),
            );
        }
    }

    fn set_default_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_popover_set_default_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_popover_set_modal(self.as_ref().to_glib_none().0, modal.into_glib());
        }
    }

    fn set_pointing_to(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_popover_set_pointing_to(self.as_ref().to_glib_none().0, rect.to_glib_none().0);
        }
    }

    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.as_ref().to_glib_none().0, position.into_glib());
        }
    }

    fn set_relative_to(&self, relative_to: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_popover_set_relative_to(
                self.as_ref().to_glib_none().0,
                relative_to.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_constrain_to_trampoline<
            P: IsA<Popover>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::constrain-to\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_constrain_to_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pointing_to_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pointing-to\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pointing_to_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_relative_to_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::relative-to\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_relative_to_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Popover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Popover")
    }
}
