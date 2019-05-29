// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Renderer(Object<ffi::GskRenderer, ffi::GskRendererClass, RendererClass>);

    match fn {
        get_type => || ffi::gsk_renderer_get_type(),
    }
}

impl Renderer {
    //pub fn new_for_surface(surface: /*Ignored*/&gdk::Surface) -> Option<Renderer> {
    //    unsafe { TODO: call ffi::gsk_renderer_new_for_surface() }
    //}
}

pub const NONE_RENDERER: Option<&Renderer> = None;

pub trait RendererExt: 'static {
    //fn get_surface(&self) -> /*Ignored*/Option<gdk::Surface>;

    fn is_realized(&self) -> bool;

    //fn realize(&self, surface: /*Ignored*/&gdk::Surface, error: /*Ignored*/Option<Error>) -> bool;

    //fn render(&self, root: &RenderNode, region: /*Ignored*/Option<&cairo::Region>);

    //fn render_texture(&self, root: &RenderNode, viewport: /*Ignored*/Option<&graphene::Rect>) -> /*Ignored*/Option<gdk::Texture>;

    fn unrealize(&self);

    fn get_property_realized(&self) -> bool;

    fn connect_property_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Renderer>> RendererExt for O {
    //fn get_surface(&self) -> /*Ignored*/Option<gdk::Surface> {
    //    unsafe { TODO: call ffi::gsk_renderer_get_surface() }
    //}

    fn is_realized(&self) -> bool {
        unsafe {
            from_glib(ffi::gsk_renderer_is_realized(self.as_ref().to_glib_none().0))
        }
    }

    //fn realize(&self, surface: /*Ignored*/&gdk::Surface, error: /*Ignored*/Option<Error>) -> bool {
    //    unsafe { TODO: call ffi::gsk_renderer_realize() }
    //}

    //fn render(&self, root: &RenderNode, region: /*Ignored*/Option<&cairo::Region>) {
    //    unsafe { TODO: call ffi::gsk_renderer_render() }
    //}

    //fn render_texture(&self, root: &RenderNode, viewport: /*Ignored*/Option<&graphene::Rect>) -> /*Ignored*/Option<gdk::Texture> {
    //    unsafe { TODO: call ffi::gsk_renderer_render_texture() }
    //}

    fn unrealize(&self) {
        unsafe {
            ffi::gsk_renderer_unrealize(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_realized(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"realized\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::realized\0".as_ptr() as *const _,
                Some(transmute(notify_realized_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::surface\0".as_ptr() as *const _,
                Some(transmute(notify_surface_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_realized_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GskRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Renderer> {
    let f: &F = &*(f as *const F);
    f(&Renderer::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_surface_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GskRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Renderer> {
    let f: &F = &*(f as *const F);
    f(&Renderer::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Renderer")
    }
}