#![no_std]
#![no_main]
#![feature(offset_of)]

use core::{
    mem::{offset_of, size_of},
    ptr::null_mut,
};

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let efi_graphics_output_protocol = locate_graphics_protocol(efi_system_table).unwrap();
    let vram_addr = efi_graphics_output_protocol.mode.frame_buffer_base;
    let vram_byte_size = efi_graphics_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        core::slice::from_raw_parts_mut(vram_addr as *mut u32, vram_byte_size / size_of::<u32>())
    };

    for e in vram {
        *e = 0xFFFFFF;
    }

    loop {}
}

fn locate_graphics_protocol<'a>(
    efi_system_table: &EfiSystemTable,
) -> EfiResult<&'a EfiGraphicsOutputProtocol<'a>> {
    let mut graphics_output_protocol = null_mut();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut(),
        &mut graphics_output_protocol as *mut *mut EfiGraphicsOutputProtocol as *mut *mut EfiVoid,
    );

    if status != EfiStatus::Success {
        return Err("Failed to locate graphics output protocol");
    }

    Ok(unsafe { &*graphics_output_protocol })
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct EfiVoid(u8);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct EfiHandle(u64);

#[repr(u64)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EfiStatus {
    Success = 0,
}

type LocateProtocol = extern "win64" fn(
    protocol: *const EfiGuid,
    registration: *const EfiVoid,
    interface: *mut *mut EfiVoid,
) -> EfiStatus;

type EfiResult<T> = core::result::Result<T, &'static str>;

#[repr(C)]
struct EfiBootServicesTable {
    _reserved0: [u64; 40],
    locate_protocol: LocateProtocol,
}

const _: () = assert!(offset_of!(EfiBootServicesTable, locate_protocol) == 320);

#[repr(C)]
struct EfiSystemTable {
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    0: 0x9042a9de,
    1: 0x23dc,
    2: 0x4a38,
    3: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EfiGuid(pub u32, pub u16, pub u16, pub [u8; 8]);

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
