// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RenderNode;
use TransformCategory;
use glib;
use glib::GString;
use glib::translate::*;
use graphene;
use gsk_sys;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Transform(Shared<gsk_sys::GskTransform>);

    match fn {
        ref => |ptr| gsk_sys::gsk_transform_ref(ptr),
        unref => |ptr| gsk_sys::gsk_transform_unref(ptr),
        get_type => || gsk_sys::gsk_transform_get_type(),
    }
}

impl Transform {
    pub fn new() -> Transform {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_new())
        }
    }

    fn equal(&self, second: &Transform) -> bool {
        unsafe {
            from_glib(gsk_sys::gsk_transform_equal(self.to_glib_none().0, second.to_glib_none().0))
        }
    }

    pub fn get_category(&self) -> TransformCategory {
        unsafe {
            from_glib(gsk_sys::gsk_transform_get_category(self.to_glib_none().0))
        }
    }

    pub fn invert(&self) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_invert(self.to_glib_full()))
        }
    }

    pub fn matrix(&self, matrix: &graphene::Matrix) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_matrix(self.to_glib_none().0, matrix.to_glib_none().0))
        }
    }

    pub fn perspective(&self, depth: f32) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_perspective(self.to_glib_none().0, depth))
        }
    }

    pub fn print(&self, string: &mut glib::String) {
        unsafe {
            gsk_sys::gsk_transform_print(self.to_glib_none().0, string.to_glib_none_mut().0);
        }
    }

    pub fn rotate(&self, angle: f32) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_rotate(self.to_glib_none().0, angle))
        }
    }

    pub fn rotate_3d(&self, angle: f32, axis: &graphene::Vec3) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_rotate_3d(self.to_glib_none().0, angle, axis.to_glib_none().0))
        }
    }

    pub fn scale(&self, factor_x: f32, factor_y: f32) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_scale(self.to_glib_none().0, factor_x, factor_y))
        }
    }

    pub fn scale_3d(&self, factor_x: f32, factor_y: f32, factor_z: f32) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_scale_3d(self.to_glib_none().0, factor_x, factor_y, factor_z))
        }
    }

    pub fn to_2d(&self) -> (f32, f32, f32, f32, f32, f32) {
        unsafe {
            let mut out_xx = mem::uninitialized();
            let mut out_yx = mem::uninitialized();
            let mut out_xy = mem::uninitialized();
            let mut out_yy = mem::uninitialized();
            let mut out_dx = mem::uninitialized();
            let mut out_dy = mem::uninitialized();
            gsk_sys::gsk_transform_to_2d(self.to_glib_none().0, &mut out_xx, &mut out_yx, &mut out_xy, &mut out_yy, &mut out_dx, &mut out_dy);
            (out_xx, out_yx, out_xy, out_yy, out_dx, out_dy)
        }
    }

    pub fn to_affine(&self) -> (f32, f32, f32, f32) {
        unsafe {
            let mut out_scale_x = mem::uninitialized();
            let mut out_scale_y = mem::uninitialized();
            let mut out_dx = mem::uninitialized();
            let mut out_dy = mem::uninitialized();
            gsk_sys::gsk_transform_to_affine(self.to_glib_none().0, &mut out_scale_x, &mut out_scale_y, &mut out_dx, &mut out_dy);
            (out_scale_x, out_scale_y, out_dx, out_dy)
        }
    }

    pub fn to_matrix(&self) -> graphene::Matrix {
        unsafe {
            let mut out_matrix = graphene::Matrix::uninitialized();
            gsk_sys::gsk_transform_to_matrix(self.to_glib_none().0, out_matrix.to_glib_none_mut().0);
            out_matrix
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_to_string(self.to_glib_none().0))
        }
    }

    pub fn to_translate(&self) -> (f32, f32) {
        unsafe {
            let mut out_dx = mem::uninitialized();
            let mut out_dy = mem::uninitialized();
            gsk_sys::gsk_transform_to_translate(self.to_glib_none().0, &mut out_dx, &mut out_dy);
            (out_dx, out_dy)
        }
    }

    pub fn transform(&self, other: Option<&Transform>) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_transform(self.to_glib_full(), other.to_glib_none().0))
        }
    }

    pub fn transform_bounds(&self, rect: &graphene::Rect) -> graphene::Rect {
        unsafe {
            let mut out_rect = graphene::Rect::uninitialized();
            gsk_sys::gsk_transform_transform_bounds(self.to_glib_none().0, rect.to_glib_none().0, out_rect.to_glib_none_mut().0);
            out_rect
        }
    }

    pub fn translate(&self, point: &graphene::Point) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_translate(self.to_glib_none().0, point.to_glib_none().0))
        }
    }

    pub fn translate_3d(&self, point: &graphene::Point3D) -> Option<Transform> {
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_translate_3d(self.to_glib_none().0, point.to_glib_none().0))
        }
    }

    pub fn node_get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_transform_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn node_get_transform(node: &RenderNode) -> Option<Transform> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_node_get_transform(node.to_glib_none().0))
        }
    }

    pub fn node_new(child: &RenderNode, transform: &Transform) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_transform_node_new(child.to_glib_none().0, transform.to_glib_none().0))
        }
    }

    pub fn parse(string: &str) -> Option<Transform> {
        assert_initialized_main_thread!();
        unsafe {
            let mut out_transform = ptr::null_mut();
            let ret = from_glib(gsk_sys::gsk_transform_parse(string.to_glib_none().0, &mut out_transform));
            if ret { Some(from_glib_full(out_transform)) } else { None }
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Transform {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Transform {}

impl fmt::Display for Transform {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}