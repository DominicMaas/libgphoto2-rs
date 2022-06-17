/// Current implementation status of the camera driver.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
enum CameraDriverStatus {
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
enum GphotoDeviceType {
    /// Traditional still camera
    GP_DEVICE_STILL_CAMERA = 0,
    /// Audio player
    GP_DEVICE_AUDIO_PLAYER = 1 << 0,
}

/// A bitmask of remote control related operations of the device.
/// Some drivers might support additional dynamic capabilities (like the PTP driver).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
enum CameraOperation {
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
