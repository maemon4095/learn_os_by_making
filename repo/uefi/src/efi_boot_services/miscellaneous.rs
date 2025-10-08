//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#miscellaneous-boot-services

use crate::{EfiGuid, EfiStatus, EfiVoid, U16Str};
use core::ptr::NonNull;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-setwatchdogtimer
pub type EfiSetWatchdogTimer = extern "efiapi" fn(
    timeout: usize,
    watchdog_code: u64,
    data_size: usize,
    watchdog_data: Option<NonNull<U16Str>>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-stall
pub type EfiStall = extern "efiapi" fn(microseconds: usize) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-copymem
pub type EfiCopyMem =
    extern "efiapi" fn(destination: NonNull<EfiVoid>, source: NonNull<EfiVoid>, length: usize);

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-setmem
pub type EfiSetMem = extern "efiapi" fn(buffer: NonNull<EfiVoid>, size: usize, value: u8);

/// https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-getnextmonotoniccount
pub type EfiGetNextMonotonicCount = extern "efiapi" fn(count: NonNull<u64>) -> EfiStatus;

/// https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-installconfigurationtable
pub type EfiInstallConfigurationTable =
    extern "efiapi" fn(guid: NonNull<EfiGuid>, table: NonNull<EfiVoid>) -> EfiStatus;

/// https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-calculatecrc32
pub type EfiCalculateCrc32 =
    extern "efiapi" fn(data: NonNull<EfiVoid>, data_size: usize, crc32: NonNull<u32>) -> EfiStatus;
