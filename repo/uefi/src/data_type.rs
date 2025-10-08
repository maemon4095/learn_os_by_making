// REF: https://uefi.org/specs/UEFI/2.10/02_Overview.html#data-types

pub mod u16str;

use core::{
    marker::{PhantomData, PhantomPinned},
    mem::size_of,
    ptr::NonNull,
};

pub use u16str::U16Str;

// REF: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiVoid {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EfiHandle(NonNull<EfiVoid>);

// REF: https://uefi.org/specs/UEFI/2.11/Apx_D_Status_Codes.html#status-codes
#[repr(transparent)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EfiStatus(usize);

impl EfiStatus {
    /// The operation completed successfully.
    pub const SUCCESS: Self = Self(0);
    /// The image failed to load.
    pub const LOAD_ERROR: Self = Self(1);
    /// A parameter was incorrect.
    pub const INVALID_PARAMETER: Self = Self(2);
    /// The operation is not supported.
    pub const UNSUPPORTED: Self = Self(3);
    /// The buffer was not the proper size for the request.
    pub const BAD_BUFFER_SIZE: Self = Self(4);
    /// The buffer is not large enough to hold the requested data. The required buffer size is returned in the appropriate parameter when this error occurs.
    pub const BUFFER_TOO_SMALL: Self = Self(5);
    /// There is no data pending upon return.
    pub const NOT_READY: Self = Self(6);
    /// The physical device reported an error while attempting the operation.
    pub const DEVICE_ERROR: Self = Self(7);
    /// The device cannot be written to.
    pub const WRITE_PROTECTED: Self = Self(8);
    /// A resource has run out.
    pub const OUT_OF_RESOURCES: Self = Self(9);
    /// An inconstancy was detected on the file system causing the operating to fail.
    pub const VOLUME_CORRUPTED: Self = Self(10);
    /// There is no more space on the file system.
    pub const VOLUME_FULL: Self = Self(11);
    /// The device does not contain any medium to perform the operation.
    pub const NO_MEDIA: Self = Self(12);
    /// The medium in the device has changed since the last access.
    pub const MEDIA_CHANGED: Self = Self(13);
    /// The item was not found.
    pub const NOT_FOUND: Self = Self(14);
    /// Access was denied.
    pub const ACCESS_DENIED: Self = Self(15);
    /// The server was not found or did not respond to the request.
    pub const NO_RESPONSE: Self = Self(16);
    /// A mapping to a device does not exist.
    pub const NO_MAPPING: Self = Self(17);
    /// The timeout time expired.
    pub const TIMEOUT: Self = Self(18);
    /// The protocol has not been started.
    pub const NOT_STARTED: Self = Self(19);
    /// The protocol has already been started.
    pub const ALREADY_STARTED: Self = Self(20);
    /// The operation was aborted.
    pub const ABORTED: Self = Self(21);
    /// An ICMP error occurred during the network operation.
    pub const ICMP_ERROR: Self = Self(22);
    /// A TFTP error occurred during the network operation.
    pub const TFTP_ERROR: Self = Self(23);
    /// A protocol error occurred during the network operation.
    pub const PROTOCOL_ERROR: Self = Self(24);
    /// The function encountered an internal version that was incompatible with a version requested by the caller.
    pub const INCOMPATIBLE_VERSION: Self = Self(25);
    /// The function was not performed due to a security violation.
    pub const SECURITY_VIOLATION: Self = Self(26);
    /// A CRC error was detected.
    pub const CRC_ERROR: Self = Self(27);
    /// Beginning or end of media was reached
    pub const END_OF_MEDIA: Self = Self(28);
    /// The end of the file was reached.
    pub const END_OF_FILE: Self = Self(31);
    /// The language specified was invalid.
    pub const INVALID_LANGUAGE: Self = Self(32);
    /// The security status of the data is unknown or compromised and the data must be updated or replaced to restore a valid security status.
    pub const COMPROMISED_DATA: Self = Self(33);
    /// There is an address conflict address allocation
    pub const IP_ADDRESS_CONFLICT: Self = Self(34);
    /// A HTTP error occurred during the network operation.
    pub const HTTP_ERROR: Self = Self(35);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiGuid(pub u32, pub u16, pub u16, pub [u8; 8]);

const _: () = assert!(size_of::<EfiGuid>() * 8 == 128);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EfiTpl(usize);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EfiEvent(*mut EfiVoid);
