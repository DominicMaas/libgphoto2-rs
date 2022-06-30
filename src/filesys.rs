use libc::{c_char, c_int, c_void, time_t};

/// Bitmask on what fields are set in the CameraFileInfo structure.
///
/// Bitmask to mark up which fields are set in the CameraFileInfo
/// structure. The other fields might be uninitialized.
/// If you set information via gp_camera_file_set_info() you
/// need to set those flags. If you retrieve information via
/// gp_camera_file_get_info() you need to check those flags.
/// They are separate for both "normal" and "preview" parts
/// and are mostly image related.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFileInfoFields {
    /// No fields set.
    GP_FILE_INFO_NONE = 0,
    /// he MIME type is set.
    GP_FILE_INFO_TYPE = 1 << 0,
    // The filesize is set.
    GP_FILE_INFO_SIZE = 1 << 2,
    /// The width is set.
    GP_FILE_INFO_WIDTH = 1 << 3,
    /// The height is set.
    GP_FILE_INFO_HEIGHT = 1 << 4,
    /// The access permissions are set.
    GP_FILE_INFO_PERMISSIONS = 1 << 5,
    /// The status is set (downloaded).
    GP_FILE_INFO_STATUS = 1 << 6,
    /// The modification time is set.
    GP_FILE_INFO_MTIME = 1 << 7,
    /// All possible fields set. Internal.
    GP_FILE_INFO_ALL = 0xFF,
}

/// Bitmask containing the file permission flags.
///
/// Possible flag values of the permission entry in the file information.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFilePermissions {
    /// No permissions.
    GP_FILE_PERM_NONE = 0,
    /// Read permissions.
    GP_FILE_PERM_READ = 1 << 0,
    /// Write permissions
    GP_FILE_PERM_DELETE = 1 << 1,
    /// Internal.
    GP_FILE_PERM_ALL = 0xFF,
}

/// Possible status values.
///
/// Bitmask of possible stati. Currently only download is supported.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraFileStatus {
    /// File is not downloaded.
    GP_FILE_STATUS_NOT_DOWNLOADED,
    /// File is already downloaded.
    GP_FILE_STATUS_DOWNLOADED,
}

/// File information of a regular file.
///
/// Contains information a regular file with fields being
/// set depending on the bitmask in the fields member.
#[repr(C)]
pub struct CameraFileInfoFile {
    /// Bitmask containing the set members.
    pub fields: CameraFileInfoFields,
    /// Status of the file.
    pub status: CameraFileStatus,
    /// Size of the file.
    pub size: u64,
    /// MIME type of the file.
    pub mime_type: [c_char; 64],
    /// Height of the file.
    pub width: u32,
    /// Width of the file.
    pub height: u32,
    /// Permissions of the file.
    pub permissions: CameraFilePermissions,
    /// Modification time of the file.
    pub mtime: time_t,
}

/// File information of a preview file.
///
/// Contains information of a preview file with fields being
/// set depending on the bitmask in the fields member.
#[repr(C)]
pub struct CameraFileInfoPreview {
    /// Bitmask containing the set members.
    pub fields: CameraFileInfoFields,
    /// Status of the preview.
    pub status: CameraFileStatus,
    /// Size of the preview.
    pub size: u64,
    ///  MIME type of the preview.
    pub mime_type: [c_char; 64],
    ///  Width of the preview.
    pub width: u32,
    /// Height of the preview.
    pub height: u32,
}

/// File information of an audio file.
///
/// Contains information of an audio file with fields being
/// set depending on the bitmask in the fields member.
#[repr(C)]
pub struct CameraFileInfoAudio {
    /// Bitmask containing the set members.
    pub fields: CameraFileInfoFields,
    ///  Status of the preview file.
    pub status: CameraFileStatus,
    /// Size of the audio file.
    pub size: u64,
    /// MIME type of the audio file.
    pub mime_type: [c_char; 64],
}

/// File information structure.
///
/// Contains the normal, preview and audio file information structures
/// for a specific file.
#[repr(C)]
pub struct CameraFileInfo {
    pub preview: CameraFileInfoPreview,
    pub file: CameraFileInfoFile,
    pub audio: CameraFileInfoAudio,
}

/// Storage information flags.
///
/// Bitmask to specify which entries of the filesystem
/// storage information is set.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraStorageInfoFields {
    /// The base directory.
    ///
    ///  Usually / if just 1 storage is attached.
    GP_STORAGEINFO_BASE = (1 << 0),
    /// Label of the filesystem.
    ///
    /// Could also be a DOS label.
    GP_STORAGEINFO_LABEL = (1 << 1),
    /// More verbose description.
    GP_STORAGEINFO_DESCRIPTION = (1 << 2),
    /// Access permissions.
    GP_STORAGEINFO_ACCESS = (1 << 3),
    /// Hardware type.
    GP_STORAGEINFO_STORAGETYPE = (1 << 4),
    /// Filesystem type.
    GP_STORAGEINFO_FILESYSTEMTYPE = (1 << 5),
    /// Maximum capacity in kbytes
    GP_STORAGEINFO_MAXCAPACITY = (1 << 6),
    /// Free space in kbytes.
    GP_STORAGEINFO_FREESPACEKBYTES = (1 << 7),
    /// Free space in images.
    GP_STORAGEINFO_FREESPACEIMAGES = (1 << 8),
}

/// Hardware storage types.
///
/// Type of hardware this storage is on. The types and values
/// are the same as the PTP standard uses (PTP_ST_xxx).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraStorageType {
    /// Unknown storage type.
    GP_STORAGEINFO_ST_UNKNOWN = 0,
    // A fixed ROM storage.
    GP_STORAGEINFO_ST_FIXED_ROM = 1,
    /// A removable ROM storage.
    GP_STORAGEINFO_ST_REMOVABLE_ROM = 2,
    /// A fixed RAM storage. (e.g. SDRAM)
    GP_STORAGEINFO_ST_FIXED_RAM = 3,
    /// A removable RAM storage. (any kind of cards etc)
    GP_STORAGEINFO_ST_REMOVABLE_RAM = 4,
}

/// Storage access modes.
///
/// The modes we can access the storage with. Uses the same
/// types and values as the PTP standard (PTP_AC_xxx).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraStorageAccessType {
    /// Storage is Read / Write.
    GP_STORAGEINFO_AC_READWRITE = 0,
    /// Storage is Ready Only.
    GP_STORAGEINFO_AC_READONLY = 1,
    /// Storage is Ready Only, but allows Delete.
    GP_STORAGEINFO_AC_READONLY_WITH_DELETE = 2,
}

/// Filesystem hierarchy types.
///
/// The type of the filesystem hierarchy the devices uses.
/// Same types and values as the PTP standard defines (PTP_FST_xxx).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum CameraStorageFilesystemType {
    /// Undefined or unknown filesystem hierarchy.
    GP_STORAGEINFO_FST_UNDEFINED = 0,
    /// Generic flat storage (all in 1 directory).
    GP_STORAGEINFO_FST_GENERICFLAT = 1,
    /// Generic tree hierarchy.
    GP_STORAGEINFO_FST_GENERICHIERARCHICAL = 2,
    /// DCIM style storage.
    GP_STORAGEINFO_FST_DCF = 3,
}

/// Storage information structue.
///
/// The type of the filesystem hierarchy the devices uses.
/// Same types and values as the PTP standard defines (PTP_FST_xxx).
#[repr(C)]
pub struct CameraStorageInformation {
    /// Bitmask of struct members that are specified.
    pub fields: CameraStorageInfoFields,
    /// Basedirectory of the storage. Will be "/" if just 1 storage on the camera.
    pub basedir: [c_char; 256],
    /// Label of the storage. Similar to DOS label.
    pub label: [c_char; 256],
    ///  Description of the storage.
    pub description: [c_char; 256],
    /// Hardware type of the storage.
    pub storage_type: CameraStorageType,
    /// Hierarchy type of the filesystem.
    pub fstype: CameraStorageFilesystemType,
    /// Access permissions.
    pub access: CameraStorageAccessType,
    /// Total capacity in kbytes.
    pub capacitykbytes: u64,
    ///  Free space in kbytes.
    pub freekbytes: u64,
    /// Free space in images (guessed by camera).
    pub freeimages: u64,
}

/// Filesystem structure, only exposed to camera drivers.
///
/// Internal structure, contents not exposed to frontends. Camera
/// drivers get these passed to filesystem related functions and
/// are supposed to use it only via the accessor functions.
#[repr(C)]
pub struct CameraFilesystem {
    __private: c_void,
}

extern "C" {
    pub fn gp_filesystem_new(fs: *mut *mut CameraFilesystem) -> c_int;
    pub fn gp_filesystem_free(fs: *mut CameraFilesystem) -> c_int;

    // TODO
}
