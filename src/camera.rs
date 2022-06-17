use libc::{c_char, c_int, c_void};

use crate::context::GPContext;

/// Object representing a camera attached to the system.
///
/// A Camera object represents a specific instance of a (physical of
/// virtual) camera attached to the system.
///
/// The abilities of this type of camera are stored in a CameraAbility
/// object.
///
/// The details of the Camera object are internal.
#[repr(C)]
pub struct Camera {
    __private: c_void,
}

/// CameraText structure used in various functions.
///
/// A text structure containing translated text returned
/// by various functions (about, manual, summary). You should
/// not assume a size.
#[repr(C)]
pub struct CameraText {
    /// Character string containing the translated text.
    pub text: [c_char; 32 * 1024],
}

///  A structure created by the capture operation.
///
/// A structure containing the folder and filename of an object
/// after a successful capture and is passed as reference to the
/// gp_camera_capture() function.
#[repr(C)]
pub struct CameraFilePath {
    ///  Name of the captured file.
    pub name: [c_char; 128],
    /// Name of the folder of the captured file.
    pub folder: [c_char; 1024],
}

/// Type of the capture to do.
///
/// Specifies the type of capture the user wants to do with the
/// gp_camera_capture() function.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraCaptureType {
    /// Capture an image.
    GP_CAPTURE_IMAGE = 0,
    ///  Capture a movie.
    GP_CAPTURE_MOVIE = 1,
    ///  Capture audio.
    GP_CAPTURE_SOUND = 2,
}

/// Specify what event we received from the camera.
///
/// Used by gp_camera_wait_for_event() to specify what
/// event happened on the camera.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraEventType {
    /// unknown and unhandled event. argument is a char* or NULL
    GP_EVENT_UNKNOWN = 0,
    /// timeout, no arguments
    GP_EVENT_TIMEOUT = 1,
    /// CameraFilePath* = file path on camfs
    GP_EVENT_FILE_ADDED = 2,
    /// CameraFilePath* = folder on camfs
    GP_EVENT_FOLDER_ADDED = 3,
    /// last capture is complete
    GP_EVENT_CAPTURE_COMPLETE = 4,
    /// CameraFilePath* = file path on camfs
    GP_EVENT_FILE_CHANGED = 5,
}

/// The camera exit function
///
/// This functions is called in the camera driver for closing the camera
/// connection. It should do the necessary cleanups of the internal camera
/// state, free allocated private structures and similar.
///
/// The driver does not need to close the #GPPort, this is done by libgphoto2
/// itself.
///
/// Implement this function if you need to any of this stuff, otherwise leave
/// it out.
pub type CameraExitFunc = extern "C" fn(camera: *mut Camera, context: *mut GPContext) -> c_int;

extern "C" {
    pub fn gp_camera_file_get_info(camera: *mut Camera) -> c_int;
    pub fn gp_camera_file_set_info(camera: *mut Camera) -> c_int;
    pub fn gp_camera_file_get(camera: *mut Camera) -> c_int;
    pub fn gp_camera_file_read(camera: *mut Camera) -> c_int;
    pub fn gp_camera_file_delete(
        camera: *mut Camera,
        folder: *const c_char,
        file: *const c_char,
        context: *mut GPContext,
    ) -> c_int;

    pub fn gp_camera_set_timeout_funcs();
    pub fn gp_camera_start_timeout();
    pub fn gp_camera_stop_timeout();
}
