use libc::{c_char, c_int};

/// Serial parity
///
/// Parity of the serial port.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum GPPortSerialParity {
    /// Parity is off (disabled)
    GP_PORT_SERIAL_PARITY_OFF = 0,
    /// Parity is even.
    GP_PORT_SERIAL_PARITY_EVEN,
    /// Parity is odd.
    GP_PORT_SERIAL_PARITY_ODD,
}

/// Maximum length of receive buffer
pub const GP_PORT_MAX_BUF_LEN: c_int = 4096;

/// Port settings for serial ports.
#[repr(C)]
pub struct GPPortSettingsSerial {
    /// The portname (/dev/ttyX)
    port: [c_char; 128],

    /// The baudrate of the device.
    speed: c_int,

    /// Parity data
    bits: GPPortSerialParity,

    /// How many stop bits are used.
    stopbits: c_int,
}

/// Port settings for USB ports.
pub struct GPPortSettingsUSB {
    /// Bulk IN endpoint used.
    inep: c_int,

    /// Bulk OUT endpoint used.
    outep: c_int,

    /// Interrupt endpoint used.
    intep: c_int,

    /// USB bConfigurationValue used.
    config: c_int,

    /// USB Interface number used.
    interface: c_int,

    /// USB Alternative Setting used.
    altsetting: c_int,

    ///  Maximum USB packetsize of the IN endpoint.
    maxpacketsize: c_int,

    /// USB Portname. Specific to lowlevel USB.
    port: [c_char; 64],
}
