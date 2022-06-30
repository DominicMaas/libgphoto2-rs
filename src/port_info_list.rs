// libgphoto2_port/gphoto2/gphoto2-port-info-list.h

use libc::c_void;

/// The gphoto port type.
///
/// Enumeration specifying the port type.
/// The enum is providing bitmasks, but most code uses it as
/// just the one specific values.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum GPPortType {
    /// No specific type associated.
    GP_PORT_NONE = 0,
    /// Serial port.
    GP_PORT_SERIAL = 1 << 0,
    /// USB port.
    GP_PORT_USB = 1 << 2,
    /// Disk / local mountpoint port.
    GP_PORT_DISK = 1 << 3,
    /// PTP/IP port.
    GP_PORT_PTPIP = 1 << 4,
    /// Direct IO to an usb mass storage device.
    GP_PORT_USB_DISK_DIRECT = 1 << 5,
    /// USB Mass Storage raw SCSI port.
    GP_PORT_USB_SCSI = 1 << 6,
    /// Generic IP address port.
    GP_PORT_IP = 1 << 7,
}

/// Information about the current port.
///
/// Specific information about the current port. Usually taken from the
/// "--port=XXXX" setting from the frontend.
///
/// This is not to be confused with the driver configurable port settings
/// in \ref GPPortSettings.
#[repr(C)]
pub struct _GPPortInfo {
    __private: c_void,
}

/// Information about the current port.
///
/// Specific information about the current port. Usually taken from the
/// "--port=XXXX" setting from the frontend.
///
/// This is not to be confused with the driver configurable port settings
/// in \ref GPPortSettings.
pub type GPPortInfo = *mut _GPPortInfo;

#[repr(C)]
pub struct GPPortInfoList {
    __private: c_void,
}
