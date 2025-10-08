//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#event-timer-and-task-priority-services

use crate::{EfiEvent, EfiGuid, EfiStatus, EfiTpl, EfiVoid};
use core::ptr::NonNull;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-createevent
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiEventType(u32);

impl From<u32> for EfiEventType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl core::ops::BitOr for EfiEventType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl EfiEventType {
    pub const TIMER: Self = Self(0x80000000);
    pub const RUNTIME: Self = Self(0x40000000);
    pub const NOTIFY_WAIT: Self = Self(0x00000100);
    pub const SIGNAL_EXIT_BOOT_SERVICES: Self = Self(0x00000201);
    pub const SIGNAL_VIRTUAL_ADDRESS_CHANGE: Self = Self(0x60000202);
    pub const NOTIFY_SIGNAL: Self = Self(0x00000200);
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-createevent
pub type EfiCreateEvent = extern "efiapi" fn(
    event_type: EfiEventType,
    notify_tpl: EfiTpl,
    notify_function: Option<EfiEventNotify>,
    notify_context: Option<NonNull<EfiVoid>>,
    event: NonNull<EfiEvent>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-createevent
pub type EfiEventNotify = extern "efiapi" fn(efi_event: EfiEvent, context: NonNull<EfiVoid>);

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-createeventex
pub type EfiCreateEventEx = extern "efiapi" fn(
    event_type: EfiEventType,
    notify_tpl: EfiTpl,
    notify_function: Option<EfiEventNotify>,
    notify_context: Option<NonNull<EfiVoid>>,
    event_group: Option<NonNull<EfiGuid>>,
    event: NonNull<EfiEvent>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-createeventex
pub mod event_group {
    use crate::EfiGuid;

    pub const EXIT_BOOT_SERVICES: EfiGuid = EfiGuid(
        0x27abf055,
        0xb1b8,
        0x4c26,
        [0x80, 0x48, 0x74, 0x8f, 0x37, 0xba, 0xa2, 0xdf],
    );

    pub const EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES: EfiGuid = EfiGuid(
        0x8be0e274,
        0x3970,
        0x4b44,
        [0x80, 0xc5, 0x1a, 0xb9, 0x50, 0x2f, 0x3b, 0xfc],
    );

    pub const EFI_EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE: EfiGuid = EfiGuid(
        0x13fa7698,
        0xc831,
        0x49c7,
        [0x87, 0xea, 0x8f, 0x43, 0xfc, 0xc2, 0x51, 0x96],
    );

    pub const EFI_EVENT_GROUP_MEMORY_MAP_CHANGE: EfiGuid = EfiGuid(
        0x78bee926,
        0x692f,
        0x48fd,
        [0x9e, 0xdb, 0x1, 0x42, 0x2e, 0xf0, 0xd7, 0xab],
    );

    pub const EFI_EVENT_GROUP_READY_TO_BOOT: EfiGuid = EfiGuid(
        0x7ce88fb3,
        0x4bd7,
        0x4679,
        [0x87, 0xa8, 0xa8, 0xd8, 0xde, 0xe5, 0xd, 0x2b],
    );

    pub const EFI_EVENT_GROUP_AFTER_READY_TO_BOOT: EfiGuid = EfiGuid(
        0x3a2a00ad,
        0x98b9,
        0x4cdf,
        [0xa4, 0x78, 0x70, 0x27, 0x77, 0xf1, 0xc1, 0xb],
    );

    pub const EFI_EVENT_GROUP_RESET_SYSTEM: EfiGuid = EfiGuid(
        0x62da6a56,
        0x13fb,
        0x485a,
        [0xa8, 0xda, 0xa3, 0xdd, 0x79, 0x12, 0xcb, 0x6b],
    );
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-closeevent
pub type EfiCloseEvent = extern "efiapi" fn(event: EfiEvent) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-signalevent
pub type EfiSignalEvent = extern "efiapi" fn(event: EfiEvent) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-waitforevent
pub type EfiWaitForEvent = extern "efiapi" fn(
    number_of_events: usize,
    event: NonNull<EfiEvent>,
    index: NonNull<usize>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-waitforevent
pub type EfiCheckEvent = extern "efiapi" fn(event: EfiEvent) -> EfiStatus;
