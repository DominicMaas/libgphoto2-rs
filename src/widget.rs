use libc::{c_char, c_float, c_int, c_void};

use crate::{camera::Camera, context::GPContext};

#[repr(C)]
pub struct CameraWidget {
    __private: c_void,
}

/// Type of the widget to be created.
///
/// The actual widget type we want to create. The type of the value
/// it supports depends on this type.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraWidgetType {
    /// This is the toplevel configuration widget. It should likely contain multiple #GP_WIDGET_SECTION entries
    GP_WIDGET_WINDOW = 0,
    /// Section widget (think Tab)
    GP_WIDGET_SECTION = 1,
    /// Text widget. | char
    GP_WIDGET_TEXT = 2,
    /// Slider widget. | float
    GP_WIDGET_RANGE = 3,
    /// Toggle widget (think check box) | int
    GP_WIDGET_TOGGLE = 4,
    /// Radio button widget. | char *
    GP_WIDGET_RADIO = 5,
    /// Menu widget (same as RADIO). | char *
    GP_WIDGET_MENU = 6,
    /// Button press widget. | CameraWidgetCallback
    GP_WIDGET_BUTTON = 7,
    /// Date entering widget. | int
    GP_WIDGET_DATE = 8,
}

/// Callback handler for Button widgets.
pub type CameraWidgetCallback =
    extern "C" fn(camera: *mut Camera, widget: *mut CameraWidget, context: *mut GPContext) -> c_int;

extern "C" {
    pub fn gp_widget_new(
        widget_type: CameraWidgetType,
        label: *const c_char,
        widget: *mut *mut CameraWidget,
    ) -> c_int;

    pub fn gp_widget_free(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_ref(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_unref(widget: *mut CameraWidget) -> c_int;

    pub fn gp_widget_append(widget: *mut CameraWidget, child: *mut CameraWidget) -> c_int;
    pub fn gp_widget_prepend(widget: *mut CameraWidget, child: *mut CameraWidget) -> c_int;

    pub fn gp_widget_count_children(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_child(
        widget: *mut CameraWidget,
        child_number: c_int,
        child: *mut *mut CameraWidget,
    ) -> c_int;

    pub fn gp_widget_get_child_by_label(
        widget: *mut CameraWidget,
        label: *const c_char,
        child: *mut *mut CameraWidget,
    ) -> c_int;
    pub fn gp_widget_get_child_by_id(
        widget: *mut CameraWidget,
        id: c_int,
        child: *mut *mut CameraWidget,
    ) -> c_int;
    pub fn gp_widget_get_child_by_name(
        widget: *mut CameraWidget,
        name: *const c_char,
        child: *mut *mut CameraWidget,
    ) -> c_int;
    pub fn gp_widget_get_root(widget: *mut CameraWidget, root: *mut *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_parent(widget: *mut CameraWidget, parent: *mut *mut CameraWidget)
        -> c_int;

    pub fn gp_widget_set_value(widget: *mut CameraWidget, value: *const c_void) -> c_int;
    pub fn gp_widget_get_value(widget: *mut CameraWidget, value: *mut c_void) -> c_int;

    pub fn gp_widget_set_name(widget: *mut CameraWidget, name: *const c_char) -> c_int;
    pub fn gp_widget_get_name(widget: *mut CameraWidget, name: *mut *const c_char) -> c_int;

    pub fn gp_widget_set_info(widget: *mut CameraWidget, info: *const c_char) -> c_int;
    pub fn gp_widget_get_info(widget: *mut CameraWidget, info: *mut *const c_char) -> c_int;

    pub fn gp_widget_get_id(widget: *mut CameraWidget, id: *mut c_int) -> c_int;
    pub fn gp_widget_get_type(
        widget: *mut CameraWidget,
        widget_type: *mut CameraWidgetType,
    ) -> c_int;
    pub fn gp_widget_get_label(widget: *mut CameraWidget, label: *mut *const c_char) -> c_int;

    pub fn gp_widget_set_range(
        range: *mut CameraWidget,
        low: c_float,
        high: c_float,
        increment: c_float,
    ) -> c_int;
    pub fn gp_widget_get_range(
        range: *mut CameraWidget,
        min: *mut c_float,
        max: *mut c_float,
        increment: *mut c_float,
    ) -> c_int;

    pub fn gp_widget_add_choice(widget: *mut CameraWidget, choice: *const c_char) -> c_int;
    pub fn gp_widget_count_choices(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_get_choice(
        widget: *mut CameraWidget,
        choice_number: c_int,
        choice: *mut *const c_char,
    ) -> c_int;

    pub fn gp_widget_changed(widget: *mut CameraWidget) -> c_int;
    pub fn gp_widget_set_changed(widget: *mut CameraWidget, changed: c_int) -> c_int;

    pub fn gp_widget_set_readonly(widget: *mut CameraWidget, readonly: c_int) -> c_int;
    pub fn gp_widget_get_readonly(widget: *mut CameraWidget, readonly: *mut c_int) -> c_int;
}
