use libc::{c_char, c_int, c_uint, c_void};

use crate::{
    abilities_list::CameraAbilities,
    context::GPContext,
    file::{CameraFile, CameraFileType},
    filesys::{CameraFileInfo, CameraStorageInformation},
    list::CameraList,
    port_info_list::GPPortInfo,
    widget::CameraWidget,
};

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

/// Get a configuration tree for the camera and its driver
///
/// A camera driver can support configuration of either its own behaviour
/// or the camera device itself. To allow a flexible driver framework,
/// the camera driver provides a generic configuration widget tree to the
/// frontend, which then renders it, allows user input and sends it back
/// via the #CameraSetConfigFunc function to have the driver configure itself
/// or the camera.
///
/// If you do not have configuration ability, there is no need to specify this
/// function.
pub type CameraGetConfigFunc = extern "C" fn(
    camera: *mut Camera,
    widget: *mut *mut CameraWidget,
    context: *mut GPContext,
) -> c_int;

/// Get a configuration widget for a specific configuration
///
/// A camera driver can support configuration of either its own behaviour
/// or the camera device itself. To allow a flexible driver framework,
/// the camera driver provides a generic configuration widget tree to the
/// frontend, which then renders it, allows user input and sends it back
/// via the #CameraSetConfigFunc function to have the driver configure itself
/// or the camera.
///
/// This specific function retrieves one specific named entry, and not the full
/// tree to allow for querying specific settings faster.
///
/// If you do not have configuration ability, there is no need to specify this
/// function.
pub type CameraGetSingleConfigFunc = extern "C" fn(
    camera: *mut Camera,
    name: *const c_char,
    widget: *mut *mut CameraWidget,
    context: *mut GPContext,
) -> c_int;

// ---- //

pub type CameraTimeoutFunc = extern "C" fn(camera: *mut Camera, context: *mut GPContext) -> c_int;

pub type CameraTimeoutStartFunc = extern "C" fn(
    camera: *mut Camera,
    timeout: c_uint,
    func: CameraTimeoutFunc,
    data: *mut c_void,
) -> c_uint;

pub type CameraTimeoutStopFunc = extern "C" fn(camera: *mut Camera, id: c_uint, data: *mut c_void);

#[repr(C)]
pub struct CameraPrivateLibrary {
    __private: c_void,
}

#[repr(C)]
pub struct CameraPrivateCore {
    __private: c_void,
}

extern "C" {
    pub fn gp_camera_new(camera: *mut *mut Camera) -> c_int;

    // Preparing initialization//

    pub fn gp_camera_set_abilities(camera: *mut Camera, abilities: CameraAbilities) -> c_int;
    pub fn gp_camera_get_abilities(camera: *mut Camera, abilities: *mut CameraAbilities) -> c_int;
    pub fn gp_camera_set_port_info(camera: *mut Camera, info: GPPortInfo) -> c_int;
    pub fn gp_camera_get_port_info(camera: *mut Camera, info: *mut GPPortInfo) -> c_int;

    // Camera speed //

    pub fn gp_camera_get_port_speed(camera: *mut Camera) -> c_int;
    pub fn gp_camera_set_port_speed(camera: *mut Camera, speed: c_int) -> c_int;

    // Initialization //

    pub fn gp_camera_autodetect(list: *mut CameraList, context: *mut GPContext) -> c_int;
    pub fn gp_camera_init(camera: *mut Camera, context: *mut GPContext) -> c_int;
    pub fn gp_camera_exit(camera: *mut Camera, context: *mut GPContext) -> c_int;

    // Operations on cameras //

    pub fn gp_camera_ref(camera: *mut Camera) -> c_int;
    pub fn gp_camera_unref(camera: *mut Camera) -> c_int;
    pub fn gp_camera_free(camera: *mut Camera) -> c_int;
    pub fn gp_camera_list_config(
        camera: *mut Camera,
        list: *mut CameraList,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_get_config(
        camera: *mut Camera,
        window: *mut *mut CameraWidget,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_set_config(
        camera: *mut Camera,
        window: *mut CameraWidget,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_get_single_config(
        camera: *mut Camera,
        name: *const c_char,
        widget: *mut *mut CameraWidget,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_set_single_config(
        camera: *mut Camera,
        name: *const c_char,
        widget: *mut CameraWidget,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_get_summary(
        camera: *mut Camera,
        summary: *mut CameraText,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_get_manual(
        camera: *mut Camera,
        manual: *mut CameraText,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_get_about(
        camera: *mut Camera,
        about: *mut CameraText,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_capture(
        camera: *mut Camera,
        capture_type: CameraCaptureType,
        path: *mut CameraFilePath,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_trigger_capture(camera: *mut Camera, context: *mut GPContext) -> c_int;
    pub fn gp_camera_capture_preview(
        camera: *mut Camera,
        file: *mut CameraFile,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_wait_for_event(
        camera: *mut Camera,
        timeout: c_int,
        eventtype: *mut CameraEventType,
        eventdata: *mut *mut c_void,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_get_storageinfo(
        camera: *mut Camera,
        sifs: *mut *mut CameraStorageInformation,
        nrofsifs: *mut c_int,
        context: *mut GPContext,
    ) -> c_int;

    // Operations on folders //

    pub fn gp_camera_folder_list_files(
        camera: *mut Camera,
        folder: *const c_char,
        list: *mut CameraList,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_folder_list_folders(
        camera: *mut Camera,
        folder: *const c_char,
        list: *mut CameraList,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_folder_delete_all(
        camera: *mut Camera,
        folder: *const c_char,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_folder_put_file(
        camera: *mut Camera,
        folder: *const c_char,
        filename: *const c_char,
        file_type: CameraFileType,
        file: *mut CameraFile,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_folder_make_dir(
        camera: *mut Camera,
        folder: *const c_char,
        name: *const c_char,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_folder_remove_dir(
        camera: *mut Camera,
        folder: *const c_char,
        name: *const c_char,
        context: *mut GPContext,
    ) -> c_int;

    // Operations on files //

    pub fn gp_camera_file_get_info(
        camera: *mut Camera,
        folder: *const c_char,
        file: *const c_char,
        info: *mut CameraFileInfo,
        contest: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_file_set_info(
        camera: *mut Camera,
        folder: *const c_char,
        file: *const c_char,
        info: CameraFileInfo,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_file_get(
        camera: *mut Camera,
        folder: *const c_char,
        file: *const c_char,
        file_type: CameraFileType,
        camera_file: *mut CameraFile,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_file_read(
        camera: *mut Camera,
        folder: *const c_char,
        file: *const c_char,
        file_type: CameraFileType,
        offset: u64,
        buf: *mut c_char,
        size: *mut u64,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_camera_file_delete(
        camera: *mut Camera,
        folder: *const c_char,
        file: *const c_char,
        context: *mut GPContext,
    ) -> c_int;

    // Some cameras need 'keep-alive-messages' //

    pub fn gp_camera_set_timeout_funcs(
        camera: *mut Camera,
        start_func: CameraTimeoutStartFunc,
        stop_func: CameraTimeoutStopFunc,
        data: *mut c_void,
    );
    pub fn gp_camera_start_timeout(
        camera: *mut Camera,
        timeout: c_uint,
        func: CameraTimeoutFunc,
    ) -> c_int;
    pub fn gp_camera_stop_timeout(camera: *mut Camera, id: c_uint);
}
