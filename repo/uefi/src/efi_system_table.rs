use core::ptr::NonNull;

use crate::efi_configuration_table::EfiConfigurationTable;
use crate::protocol::simple_text::{SimpleTextInputProtocol, SimpleTextOutputProtocol};
use crate::{EfiBootServices, EfiRuntimeServices};

use super::u16str::U16Str;
use super::{EfiHandle, EfiTableHeader};

// REF: https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: NonNull<U16Str>,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    console_in: NonNull<SimpleTextInputProtocol>,
    console_out_handle: EfiHandle,
    console_out: NonNull<SimpleTextOutputProtocol>,
    standard_error_handle: EfiHandle,
    std_err: NonNull<SimpleTextOutputProtocol>,
    runtime_services: NonNull<EfiRuntimeServices>,
    boot_services: NonNull<EfiBootServices>,
    number_of_table_entries: usize,
    efi_configuration_table: NonNull<EfiConfigurationTable>,
}
