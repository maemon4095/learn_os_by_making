pub mod device_path;
pub mod graphics;
pub mod simple_text;

use crate::{EfiGuid, EfiResult, EfiStatus, EfiVoid};
use core::{
    mem::{offset_of, size_of},
    ptr::null_mut,
};

pub type LocateProtocol = extern "win64" fn(
    protocol: *const EfiGuid,
    registration: *const EfiVoid,
    interface: *mut *mut EfiVoid,
) -> EfiStatus;

#[repr(C)]
pub struct EfiBootServicesTable {
    _reserved0: [u64; 40],
    locate_protocol: LocateProtocol,
}

const _: () = assert!(offset_of!(EfiBootServicesTable, locate_protocol) == 320);

#[repr(C)]
struct EfiSystemTable {
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid(
    0x9042a9de,
    0x23dc,
    0x4a38,
    [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocol<'a> {
    reserved: [u64; 3],
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mod: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputProtocolPixelInfo,
    pub size_of_info: u64,
    pub frame_buffer_base: usize,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolPixelInfo {
    version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    _padding0: [u32; 5],
    pub pixels_per_scal_line: u32,
}

const _: () = assert!(size_of::<EfiGraphicsOutputProtocolPixelInfo>() == 36);

fn locate_graphics_protocol<'a>(
    efi_system_table: &EfiSystemTable,
) -> EfiResult<&'a EfiGraphicsOutputProtocol<'a>> {
    let mut graphics_output_protocol = null_mut();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut(),
        &mut graphics_output_protocol as *mut *mut EfiGraphicsOutputProtocol as *mut *mut EfiVoid,
    );

    if status != EfiStatus::SUCCESS {
        return Err("Failed to locate graphics output protocol");
    }

    Ok(unsafe { &*graphics_output_protocol })
}
