//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#event-timer-and-task-priority-services

use crate::{EfiEvent, EfiStatus};

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-settimer
pub type EfiSetTimer =
    extern "C" fn(event: EfiEvent, time_type: EfiTimerDelay, trigger_time: u64) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-settimer
#[derive(Debug)]
#[repr(u32)]
pub enum EfiTimerDelay {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}
