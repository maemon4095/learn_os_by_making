pub mod event;
pub mod image;
pub mod memory_allocation;
pub mod miscellaneous;
pub mod protocol_handler;
pub mod task_priority;
pub mod timer;

use crate::{
    efi_boot_services::{
        event::{
            EfiCheckEvent, EfiCloseEvent, EfiCreateEvent, EfiCreateEventEx, EfiSignalEvent,
            EfiWaitForEvent,
        },
        image::{EfiExit, EfiExitBootServices, EfiLoadImage, EfiStartImage, EfiUnloadImage},
        memory_allocation::{
            EfiAllocatePages, EfiAllocatePool, EfiFreePages, EfiFreePool, EfiGetMemoryMap,
        },
        miscellaneous::{
            EfiCalculateCrc32, EfiCopyMem, EfiGetNextMonotonicCount, EfiInstallConfigurationTable,
            EfiSetMem, EfiSetWatchdogTimer, EfiStall,
        },
        protocol_handler::{
            EfiCloseProtocol, EfiConnectController, EfiDisconnectController, EfiHandleProtocol,
            EfiInstallMultipleProtocolInterfaces, EfiInstallProtocolInterface, EfiLocateDevicePath,
            EfiLocateHandle, EfiLocateHandleBuffer, EfiLocateProtocol, EfiOpenProtocol,
            EfiOpenProtocolInformation, EfiProtocolsPerHandle, EfiRegisterProtocolNotify,
            EfiReinstallProtoclInterface, EfiUninstallMultipleProtocolInterfaces,
            EfiUninstallProtocolInterface,
        },
        task_priority::{EfiRaiseTpl, EfiRestoreTpl},
        timer::EfiSetTimer,
    },
    EfiTableHeader, EfiVoid,
};

pub use event::event_group;
pub use memory_allocation::{
    EfiAllocateType, EfiMemoryAttribute, EfiMemoryType, EfiPhysicalAddress, EfiVirtualAddress,
};
pub use protocol_handler::{
    EfiLocateSearchType, EfiOpenProtocolAttributes, EfiOpenProtocolInformationEntry,
};
pub use timer::EfiTimerDelay;

/// REF: https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-boot-services-table
#[repr(C)]
#[derive(Debug)]
pub struct EfiBootServices {
    hdr: EfiTableHeader,

    //
    // Task Priority Services
    //
    raise_tpl: EfiRaiseTpl,
    restore_tpl: EfiRestoreTpl,

    //
    // Memory Services
    //
    allocate_pages: EfiAllocatePages,
    free_pages: EfiFreePages,
    get_memory_map: EfiGetMemoryMap,
    allocate_pool: EfiAllocatePool,
    free_pool: EfiFreePool,

    //
    // Event & Timer Services
    //
    create_event: EfiCreateEvent,
    set_timer: EfiSetTimer,
    wait_for_event: EfiWaitForEvent,
    signal_event: EfiSignalEvent,
    close_event: EfiCloseEvent,
    check_event: EfiCheckEvent,

    //
    // Protocol Handler Services
    //
    install_protocol_interface: EfiInstallProtocolInterface,
    reinstall_protocol_interface: EfiReinstallProtoclInterface,
    uninsatall_protocol_interface: EfiUninstallProtocolInterface,
    handle_protocol: EfiHandleProtocol,
    _reserved: *mut EfiVoid,
    register_protocol_notify: EfiRegisterProtocolNotify,
    locate_handle: EfiLocateHandle,
    locate_device_path: EfiLocateDevicePath,
    install_configuration_table: EfiInstallConfigurationTable,

    //
    // Image Services
    //
    load_image: EfiLoadImage,
    start_image: EfiStartImage,
    exit: EfiExit,
    unload_image: EfiUnloadImage,
    exit_boot_services: EfiExitBootServices,

    //
    // Miscellaneous Services
    //
    get_next_monotonic_count: EfiGetNextMonotonicCount,
    stall: EfiStall,
    set_watchdog_timer: EfiSetWatchdogTimer,

    //
    // DriverSupport Services
    //
    connect_controller: EfiConnectController,
    disconnect_controller: EfiDisconnectController,

    //
    // Open and Close Protocol Services
    //
    open_protocol: EfiOpenProtocol,
    close_protocol: EfiCloseProtocol,
    open_protocol_information: EfiOpenProtocolInformation,

    //
    // Library Services
    //
    protocols_per_handle: EfiProtocolsPerHandle,
    locate_handle_buffer: EfiLocateHandleBuffer,
    locate_protocol: EfiLocateProtocol,
    install_multiple_protocol_interfaces: EfiInstallMultipleProtocolInterfaces,
    uninstall_multiple_protocol_interfaces: EfiUninstallMultipleProtocolInterfaces,

    //
    // 32-bit CRC Services
    //
    calcurate_crc32: EfiCalculateCrc32,

    //
    // Miscellaneous Services
    //
    copy_mem: EfiCopyMem,
    set_mem: EfiSetMem,
    create_event_ex: EfiCreateEventEx,
}
