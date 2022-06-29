use libc::{c_char, c_int, c_void};

use crate::{context::GPContext, list::CameraList, port_info_list::GPPortType};

/// Current implementation status of the camera driver.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraDriverStatus {
    /// Driver is production ready.
    GP_DRIVER_STATUS_PRODUCTION = 0,
    /// Driver is beta quality.
    GP_DRIVER_STATUS_TESTING = 1,
    /// Driver is alpha quality and might even not work.
    GP_DRIVER_STATUS_EXPERIMENTAL = 2,
    /// Driver is no longer recommended to use and will be removed.
    GP_DRIVER_STATUS_DEPRECATED = 3,
}

/// Type of the device represented. Currently we have Still Cameras
/// and MTP Audio Players.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum GphotoDeviceType {
    /// Traditional still camera
    GP_DEVICE_STILL_CAMERA = 0,
    /// Audio player
    GP_DEVICE_AUDIO_PLAYER = 1 << 0,
}

/// A bitmask of remote control related operations of the device.
/// Some drivers might support additional dynamic capabilities (like the PTP driver).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraOperation {
    /// No remote control operation supported.
    GP_OPERATION_NONE = 0,
    /// Capturing images supported.
    GP_OPERATION_CAPTURE_IMAGE = 1 << 0,
    /// Capturing videos supported.
    GP_OPERATION_CAPTURE_VIDEO = 1 << 1,
    /// Capturing audio supported.
    GP_OPERATION_CAPTURE_AUDIO = 1 << 2,
    /// Capturing image previews supported.
    GP_OPERATION_CAPTURE_PREVIEW = 1 << 3,
    /// Camera and Driver configuration supported.
    GP_OPERATION_CONFIG = 1 << 4,
    /// Camera can trigger capture and wait for events.
    GP_OPERATION_TRIGGER_CAPTURE = 1 << 5,
}

/// A bitmask of image related operations of the device.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFileOperation {
    /// No special file operations, just download.
    GP_FILE_OPERATION_NONE = 0,
    /// Deletion of files is possible.
    GP_FILE_OPERATION_DELETE = 1 << 1,
    /// Previewing viewfinder content is possible.
    GP_FILE_OPERATION_PREVIEW = 1 << 3,
    /// Raw retrieval is possible (used by non-JPEG cameras)
    GP_FILE_OPERATION_RAW = 1 << 4,
    /// Audio retrieval is possible.
    GP_FILE_OPERATION_AUDIO = 1 << 5,
    /// EXIF retrieval is possible.
    GP_FILE_OPERATION_EXIF = 1 << 6,
}

/// A bitmask of filesystem related operations of the device.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFolderOperation {
    /// No special filesystem operation.
    GP_FOLDER_OPERATION_NONE = 0,
    GP_FOLDER_OPERATION_DELETE_ALL = 1 << 0,
    /// Upload of files to the device possible.
    GP_FOLDER_OPERATION_PUT_FILE = 1 << 1,
    /// Making directories on the device possible.
    GP_FOLDER_OPERATION_MAKE_DIR = 1 << 2,
    /// Removing directories from the device possible.
    GP_FOLDER_OPERATION_REMOVE_DIR = 1 << 3,
}

/// Describes the properties of a specific camera.
///
/// The internals of this structures are used extensively by the
/// camlibs, but the status regarding use by frontends is questionable.
#[repr(C)]
pub struct CameraAbilities {
    /// name of camera model
    pub model: [c_char; 128],

    /// driver quality
    pub status: CameraDriverStatus,

    /// Supported port types.
    pub port: GPPortType,

    /// Supported serial port speeds (terminated with a value of 0).
    pub speed: [c_int; 64],

    /// Camera operation funcs
    pub operations: CameraOperation,

    /// Camera file op funcs
    pub file_operations: CameraFileOperation,

    /// Camera folder op funcs
    pub folder_operations: CameraFolderOperation,

    /// USB Vendor D
    pub usb_vendor: c_int,

    /// USB Product ID
    pub usb_product: c_int,

    ///  USB device class
    pub usb_class: c_int,

    /// USB device subclass
    pub usb_subclass: c_int,

    /// USB device protocol
    pub usb_protocol: c_int,

    /// (Internal) library filename
    pub library: [c_char; 1024],

    /// (Internal) camera ID name
    pub id: [c_char; 1024],

    /// Device type.
    pub device_type: GphotoDeviceType,

    reserved2: c_int,
    reserved3: c_int,
    reserved4: c_int,
    reserved5: c_int,
    reserved6: c_int,
    reserved7: c_int,
    reserved8: c_int,
}

/// List of supported camera models including their abilities
///
/// The internals of this list are hidden - use the access functions.
#[repr(C)]
pub struct CameraAbilitiesList {
    __private: c_void,
}

extern "C" {
    pub fn gp_abilities_list_new(list: *mut *mut CameraAbilitiesList) -> c_int;
    pub fn gp_abilities_list_free(list: *mut CameraAbilitiesList) -> c_int;

    pub fn gp_abilities_list_load(list: *mut CameraAbilitiesList, context: *mut GPContext)
        -> c_int;
    pub fn gp_abilities_list_load_dir(
        list: *mut CameraAbilitiesList,
        dir: *const c_char,
        context: *mut GPContext,
    ) -> c_int;
    pub fn gp_abilities_list_reset(list: *mut CameraAbilitiesList) -> c_int;

    pub fn gp_abilities_list_detect(
        list: *mut CameraAbilitiesList,
        info_list: *mut GPPortInfoList,
        l: *mut CameraList,
        context: *mut GPContext,
    ) -> c_int;

    pub fn gp_abilities_list_append(
        list: *mut CameraAbilitiesList,
        abilities: CameraAbilities,
    ) -> c_int;

    pub fn gp_abilities_list_count(list: *mut CameraAbilitiesList) -> c_int;

    pub fn gp_abilities_list_lookup_model(
        list: *mut CameraAbilitiesList,
        model: *const c_char,
    ) -> c_int;

    pub fn gp_abilities_list_get_abilities(
        list: *mut CameraAbilitiesList,
        index: c_int,
        abilities: *mut CameraAbilities,
    ) -> c_int;

    pub fn gp_message_codeset(_: *const c_char) -> *const c_char;

    pub fn gp_init_localedir(localedir: *const c_char) -> c_int;
}
