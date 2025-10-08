//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#image-services

use core::ptr::NonNull;

use crate::{
    efi_system_table::EfiSystemTable, protocol::device_path::EfiDevicePathProtocol, EfiHandle,
    EfiStatus, EfiVoid, U16Str,
};

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-loadimage
pub type EfiLoadImage = extern "efiapi" fn(
    boot_policy: bool,
    parent_image_handle: EfiHandle,
    device_path: Option<NonNull<EfiDevicePathProtocol>>,
    source_buffer: Option<NonNull<EfiVoid>>,
    source_size: usize,
    image_handle: NonNull<EfiHandle>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-startimage
pub type EfiStartImage = extern "efiapi" fn(
    image_handle: EfiHandle,
    exit_data_size: NonNull<usize>,
    exit_data: Option<NonNull<*mut U16Str>>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-unloadimage
pub type EfiUnloadImage = extern "efiapi" fn(image_handle: EfiHandle) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-image-entry-point
pub type EfiImageEntryPoint =
    extern "efiapi" fn(image_handle: EfiHandle, system_table: NonNull<EfiSystemTable>) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-exit
pub type EfiExit = extern "efiapi" fn(
    image_handle: EfiHandle,
    exit_status: EfiStatus,
    exit_data_size: usize,
    exit_data: Option<NonNull<U16Str>>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-exitbootservices
pub type EfiExitBootServices =
    extern "efiapi" fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus;
