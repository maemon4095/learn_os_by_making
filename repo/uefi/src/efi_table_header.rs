use super::EfiRevision;

// REF: https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id4
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EfiTableHeader {
    signeture: u64,
    revision: EfiRevision,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}
