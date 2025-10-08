//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#event-timer-and-task-priority-services

use crate::EfiTpl;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-raisetpl
pub type EfiRaiseTpl = extern "efiapi" fn(new_tpl: EfiTpl) -> EfiTpl;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-restoretpl
pub type EfiRestoreTpl = extern "efiapi" fn(old_tpl: EfiTpl);
