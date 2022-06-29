use libc::{c_char, c_int};

/// Everything is OK
///
/// Note that this is also the value 0, and every error is negative (lower).
pub const GP_OK: c_int = 0;

/// Generic Error
pub const GP_ERROR: c_int = -1;

/// Bad parameters passed
pub const GP_ERROR_BAD_PARAMETERS: c_int = -2;

/// Out of memory
pub const GP_ERROR_NO_MEMORY: c_int = -3;

/// Error in the camera driver
pub const GP_ERROR_LIBRARY: c_int = -4;

/// Unknown libgphoto2 port passed
pub const GP_ERROR_UNKNOWN_PORT: c_int = -5;

/// Functionality not supported
pub const GP_ERROR_NOT_SUPPORTED: c_int = -6;

/// Generic I/O error
pub const GP_ERROR_IO: c_int = -7;

/// Buffer overflow of internal structure
pub const GP_ERROR_FIXED_LIMIT_EXCEEDED: c_int = -8;

/// Operation timed out
pub const GP_ERROR_TIMEOUT: c_int = -10;

/// Serial ports not supported
pub const GP_ERROR_IO_SUPPORTED_SERIAL: c_int = -20;

/// USB ports not supported
pub const GP_ERROR_IO_SUPPORTED_USB: c_int = -21;

/// Error initializing I/O
pub const GP_ERROR_IO_INIT: c_int = -31;

/// I/O during read
pub const GP_ERROR_IO_READ: c_int = -34;

/// I/O during write
pub const GP_ERROR_IO_WRITE: c_int = -35;

/// I/O during update of settings
pub const GP_ERROR_IO_UPDATE: c_int = -37;

/// Specified serial speed not possible.
pub const GP_ERROR_IO_SERIAL_SPEED: c_int = -41;

/// Error during USB Clear HALT
pub const GP_ERROR_IO_USB_CLEAR_HALT: c_int = -51;

/// Error when trying to find USB device
pub const GP_ERROR_IO_USB_FIND: c_int = -52;

/// Error when trying to claim the USB device
pub const GP_ERROR_IO_USB_CLAIM: c_int = -53;

/// Error when trying to lock the device
pub const GP_ERROR_IO_LOCK: c_int = -60;

/// Unspecified error when talking to HAL
pub const GP_ERROR_HAL: c_int = -70;

/// Corrupted data received
///
/// Data is corrupt. This error is reported by camera drivers if corrupted
/// data has been received that can not be automatically handled. Normally,
/// drivers will do everything possible to automatically recover from this
/// error.
pub const GP_ERROR_CORRUPTED_DATA: c_int = -102;

/// File already exists
///
/// An operation failed because a file existed. This error is reported for
/// example when the user tries to create a file that already exists.
pub const GP_ERROR_FILE_EXISTS: c_int = -103;

/// Specified camera model was not found
///
/// The specified model could not be found. This error is reported when
/// the user specified a model that does not seem to be supported by
/// any driver.
pub const GP_ERROR_MODEL_NOT_FOUND: c_int = -105;

/// Specified directory was not found
///
/// The specified directory could not be found. This error is reported when
/// the user specified a directory that is non-existent.
pub const GP_ERROR_DIRECTORY_NOT_FOUND: c_int = -107;

/// Specified file was not found
///
/// The specified file could not be found. This error is reported when
/// the user wants to access a file that is non-existent.
pub const GP_ERROR_FILE_NOT_FOUND: c_int = -108;

/// Specified directory already exists
///
/// The specified directory already exists. This error is reported for example
/// when the user wants to create a directory that already exists.
pub const GP_ERROR_DIRECTORY_EXISTS: c_int = -109;

/// The camera is already busy
///
/// Camera I/O or a command is in progress.
pub const GP_ERROR_CAMERA_BUSY: c_int = -110;

/// Path is not absolute
///
/// The specified path is not absolute. This error is reported when the user
/// specifies paths that are not absolute, i.e. paths like "path/to/directory".
/// As a rule of thumb, in gphoto2, there is nothing like relative paths.
pub const GP_ERROR_PATH_NOT_ABSOLUTE: c_int = -111;

/// Cancellation successful.
///
/// A cancellation requestion by the frontend via progress callback and
/// GP_CONTEXT_FEEDBACK_CANCEL was successful and the transfer has been aborted.
pub const GP_ERROR_CANCEL: c_int = -112;

/// Unspecified camera error
///
/// The camera reported some kind of error. This can be either a
/// photographic error, such as failure to autofocus, underexposure, or
/// violating storage permission, anything else that stops the camera
/// from performing the operation.
pub const GP_ERROR_CAMERA_ERROR: c_int = -113;

/// Unspecified failure of the operating system
///
/// There was some sort of OS error in communicating with the camera,
/// e.g. lack of permission for an operation.
pub const GP_ERROR_OS_FAILURE: c_int = -114;

/// Not enough space
///
/// There was not enough free space when uploading a file.
pub const GP_ERROR_NO_SPACE: c_int = -115;

extern "C" {
    pub fn gp_result_as_string(result: c_int) -> *const c_char;
}
