use libc::c_int;

use crate::{
    abilities_list::CameraAbilitiesList,
    camera::{Camera, CameraText},
    context::GPContext,
};

/// Returns a unique id for the camera driver.
///
/// id: a #CameraText
///
/// returns a gphoto2 error code
pub type CameraLibraryIdFunc = extern "C" fn(id: *mut CameraText) -> c_int;

/// Adds the abilities of the supported models to the supplied list.
pub type CameraLibraryAbilitiesFunc = extern "C" fn(list: *mut CameraAbilitiesList) -> c_int;

/// Initializes the camera.
pub type CameraLibraryInitFunc =
    extern "C" fn(camera: *mut Camera, context: *mut GPContext) -> c_int;

extern "C" {
    pub fn camera_id(id: *mut CameraText) -> c_int;
    pub fn camera_abilities(list: *mut CameraAbilitiesList) -> c_int;
    pub fn camera_init(camera: *mut Camera, context: *mut GPContext) -> c_int;
}
