//! REF: https://uefi.org/specs/UEFI/2.10/10_Protocols_Device_Path_Protocol.html#efi-device-path-protocol

use crate::EfiGuid;

pub const EFI_DEVICE_PATH_PROTOCOL_GUID: EfiGuid = EfiGuid(
    0x09576e91,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub struct EfiDevicePathProtocol {
    ty: u8,
    subty: u8,
    length: [u8; 2],
}
