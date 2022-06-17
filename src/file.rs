use libc::{c_char, c_int, c_uchar, c_void, time_t};

const_cstr! {
    GP_MIME_TXT         = "text/plain";
    GP_MIME_WAV         = "audio/wav";
    GP_MIME_RAW         = "image/x-raw";
    GP_MIME_PNG         = "image/png";
    GP_MIME_PGM         = "image/x-portable-graymap";
    GP_MIME_PPM         = "image/x-portable-pixmap";
    GP_MIME_PNM         = "image/x-portable-anymap";
    GP_MIME_JPEG        = "image/jpeg";
    GP_MIME_TIFF        = "image/tiff";
    GP_MIME_BMP         = "image/bmp";
    GP_MIME_QUICKTIME   = "video/quicktime";
    GP_MIME_AVI         = "video/x-msvideo";
    GP_MIME_CRW         = "image/x-canon-raw";
    GP_MIME_CR2         = "image/x-canon-cr2";
    GP_MIME_CR3         = "image/x-canon-cr3";
    GP_MIME_NEF         = "image/x-nikon-nef";
    GP_MIME_UNKNOWN     = "application/octet-stream";
    GP_MIME_EXIF        = "application/x-exif";
    GP_MIME_MP3         = "audio/mpeg";
    GP_MIME_OGG         = "application/ogg";
    GP_MIME_WMA         = "audio/x-wma";
    GP_MIME_ASF         = "audio/x-asf";
    GP_MIME_MPEG        = "video/mpeg";
    GP_MIME_AVCHD       = "video/mp2t";
    GP_MIME_RW2         = "image/x-panasonic-raw2";
    GP_MIME_ARW         = "image/x-sony-arw";
}

/// The type of view on the specified file.
///
/// Specifies the file of the current file, usually passed
/// to the gp_camera_file_get() and gp_camera_file_put()
/// functions. This is useful for multiple views of one
/// file, like that an single image file has "raw", "normal",
/// "exif" and "preview" views, or a media file has "normal"
/// and "metadata" file views.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFileType {
    /// A preview of an image.
    GP_FILE_TYPE_PREVIEW = 0,
    /// The regular normal data of a file.
    GP_FILE_TYPE_NORMAL = 1,
    /// The raw mode of a file, for instance the raw bayer data for cameras
    /// where postprocessing is done in the driver. The RAW files of modern
    /// DSLRs are GP_FILE_TYPE_NORMAL usually.
    GP_FILE_TYPE_RAW = 2,
    /// The audio view of a file. Perhaps an embedded comment or similar.
    GP_FILE_TYPE_AUDIO = 3,
    /// The embedded EXIF data of an image.
    GP_FILE_TYPE_EXIF = 4,
    /// The metadata of a file, like Metadata of files on MTP devices.
    GP_FILE_TYPE_METADATA = 5,
}

/// File storage type.
///
/// The file storage type. Only used internally for now, but might
/// be exposed later on. See gp_file_new() and gp_file_new_from_fd().
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFileAccessType {
    /// File is in system memory.
    GP_FILE_ACCESSTYPE_MEMORY = 0,
    /// File is associated with a UNIX filedescriptor.
    GP_FILE_ACCESSTYPE_FD = 1,
    /// File is associated with a programmatic handler.
    GP_FILE_ACCESSTYPE_HANDLER = 2,
}

#[repr(C)]
pub struct CameraFileHandler {
    size: extern "C" fn(r#priv: *mut c_void, size: *mut u64) -> c_int,
    read: extern "C" fn(r#priv: *mut c_void, data: *mut c_uchar, len: *mut u64) -> c_int,
    write: extern "C" fn(r#priv: *mut c_void, data: *mut c_uchar, len: *mut u64) -> c_int,
}

///  File structure.
///
/// The internals of the CameraFile struct are private, please use
/// the accessor functions.
#[repr(C)]
pub struct CameraFile {
    __private: c_void,
}

extern "C" {
    pub fn gp_file_new(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_new_from_fd(file: *mut *mut CameraFile, fd: c_int) -> c_int;
    pub fn gp_file_new_from_handler(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_ref(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_unref(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_free(file: *mut *mut CameraFile) -> c_int;

    pub fn gp_file_set_name(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_get_name(file: *mut *mut CameraFile) -> c_int;

    pub fn gp_file_set_mime_type(file: *mut *mut CameraFile, mime_type: *const c_char) -> c_int;
    pub fn gp_file_get_mime_type(
        file: *mut *mut CameraFile,
        mime_type: *mut *const c_char,
    ) -> c_int;

    pub fn gp_file_set_mtime(file: *mut *mut CameraFile, mtime: time_t) -> c_int;
    pub fn gp_file_get_mtime(file: *mut *mut CameraFile, mtime: *mut time_t) -> c_int;

    pub fn gp_file_detect_mime_type(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_adjust_name_for_mime_type(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_get_name_by_type(
        file: *mut *mut CameraFile,
        base_name: *const c_char,
        file_type: CameraFileType,
        new_name: *mut *const c_char,
    ) -> c_int;

    pub fn gp_file_set_data_and_size(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_get_data_and_size(file: *mut *mut CameraFile) -> c_int;
}
