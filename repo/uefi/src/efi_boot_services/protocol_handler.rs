//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#protocol-handler-services

use crate::{
    protocol::device_path::EfiDevicePathProtocol, EfiEvent, EfiGuid, EfiHandle, EfiStatus, EfiVoid,
};
use core::ptr::NonNull;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-installprotocolinterface
pub type EfiInstallProtocolInterface = extern "efiapi" fn(
    handle: NonNull<EfiHandle>,
    protocol: NonNull<EfiGuid>,
    interface_type: EfiInterfaceType,
    inteface: NonNull<EfiVoid>,
) -> EfiStatus;

#[derive(Debug)]
#[repr(u32)]
pub enum EfiInterfaceType {
    EfiNativeInterface,
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-uninstallprotocolinterface
pub type EfiUninstallProtocolInterface = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: NonNull<EfiGuid>,
    interface: NonNull<EfiVoid>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-reinstallprotocolinterface
pub type EfiReinstallProtoclInterface = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: NonNull<EfiGuid>,
    old_interface: NonNull<EfiVoid>,
    new_interface: NonNull<EfiVoid>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-registerprotocolnotify
pub type EfiRegisterProtocolNotify = extern "efiapi" fn(
    protocol: NonNull<EfiGuid>,
    event: EfiEvent,
    registration: NonNull<*mut EfiVoid>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-locatehandle
pub type EfiLocateHandle = extern "efiapi" fn(
    search_type: EfiLocateSearchType,
    protocol: Option<NonNull<EfiGuid>>,
    search_key: Option<NonNull<EfiVoid>>,
    buffer_size: NonNull<usize>,
    buffer: NonNull<EfiHandle>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-locatehandle
#[derive(Debug)]
#[repr(u32)]
pub enum EfiLocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-handleprotocol
pub type EfiHandleProtocol = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: NonNull<EfiGuid>,
    interface: NonNull<*mut EfiVoid>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-locatedevicepath
pub type EfiLocateDevicePath = extern "efiapi" fn(
    protocol: NonNull<EfiGuid>,
    device_path: NonNull<NonNull<EfiDevicePathProtocol>>,
    device: NonNull<EfiHandle>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-openprotocol
pub type EfiOpenProtocol = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: NonNull<EfiGuid>,
    interface: Option<NonNull<*mut EfiVoid>>,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: EfiOpenProtocolAttributes,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-openprotocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EfiOpenProtocolAttributes(u32);

impl core::ops::BitOr for EfiOpenProtocolAttributes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl EfiOpenProtocolAttributes {
    /// Used in the implementation of EFI_BOOT_SERVICES.HandleProtocol() . Since EFI_BOOT_SERVICES.OpenProtocol() performs the same function as HandleProtocol() with additional functionality, HandleProtocol() can simply call OpenProtocol() with this Attributes value.
    pub const BY_HANDLE_PROTOCOL: Self = Self(0x00000001);
    /// Used by a driver to get a protocol interface from a handle. Care must be taken when using this open mode because the driver that opens a protocol interface in this manner will not be informed if the protocol interface is uninstalled or reinstalled. The caller is also not required to close the protocol interface with EFI_BOOT_SERVICES.CloseProtocol() .
    pub const GET_PROTOCOL: Self = Self(0x00000002);
    /// Used by a driver to test for the existence of a protocol interface on a handle. Interface is optional for this attribute value, so it is ignored, and the caller should only use the return status code. The caller is also not required to close the protocol interface with CloseProtocol().
    pub const TEST_PROTOCOL: Self = Self(0x00000004);
    /// Used by bus drivers to show that a protocol interface is being used by one of the child controllers of a bus. This information is used by the boot service EFI_BOOT_SERVICES.ConnectController() to recursively connect all child controllers and by the boot service EFI_BOOT_SERVICES.DisconnectController() to get the list of child controllers that a bus driver created.
    pub const BY_CHILD_CONTROLLER: Self = Self(0x00000008);
    ///  Used by a driver to gain access to a protocol interface. When this mode is used, the driver’s Stop() function will be called by EFI_BOOT_SERVICES.DisconnectController() if the protocol interface is reinstalled or uninstalled. Once a protocol interface is opened by a driver with this attribute, no other drivers will be allowed to open the same protocol interface with the BY_DRIVER attribute.
    pub const BY_DRIVER: Self = Self(0x00000010);
    /// Used by applications to gain exclusive access to a protocol interface. If any drivers have the protocol interface opened with an attribute of BY_DRIVER, then an attempt will be made to remove them by calling the driver’s Stop() function.
    pub const EXCLUSIVE: Self = Self(0x00000020);

    /// Used by a driver to gain exclusive access to a protocol interface. If any other drivers have the protocol interface opened with an attribute of BY_DRIVER, then an attempt will be made to remove them with DisconnectController().
    pub const BY_DRIVER_OR_EXCLUSIVE: Self = Self(Self::BY_DRIVER.0 | Self::EXCLUSIVE.0);
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-closeprotocol
pub type EfiCloseProtocol = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: NonNull<EfiGuid>,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-openprotocolinformation
pub type EfiOpenProtocolInformation = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: NonNull<EfiGuid>,
    entry_buffer: NonNull<*mut EfiOpenProtocolInformationEntry>,
    entry_count: NonNull<usize>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-openprotocolinformation
#[derive(Debug)]
#[repr(C)]
pub struct EfiOpenProtocolInformationEntry {
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: EfiOpenProtocolAttributes,
    open_count: u32,
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-connectcontroller
pub type EfiConnectController = extern "efiapi" fn(
    controller_handle: EfiHandle,
    driver_image_handle: Option<NonNull<EfiHandle>>,
    remaining_device_path: Option<NonNull<EfiDevicePathProtocol>>,
    recursive: bool,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-disconnectcontroller
pub type EfiDisconnectController = extern "efiapi" fn(
    controller_handle: EfiHandle,
    driver_image_handle: Option<EfiHandle>,
    child_handle: Option<EfiHandle>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-protocolsperhandle
pub type EfiProtocolsPerHandle = extern "efiapi" fn(
    handle: EfiHandle,
    protocol_buffer: NonNull<*mut *mut EfiGuid>,
    protocol_buffer_count: NonNull<usize>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-locatehandlebuffer
pub type EfiLocateHandleBuffer = extern "efiapi" fn(
    search_type: EfiLocateSearchType,
    protocol: Option<NonNull<EfiGuid>>,
    search_key: Option<NonNull<EfiVoid>>,
    no_handles: NonNull<usize>,
    buffer: NonNull<*mut EfiHandle>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-locateprotocol
pub type EfiLocateProtocol = extern "efiapi" fn(
    protocol: NonNull<EfiGuid>,
    registrration: Option<NonNull<EfiVoid>>,
    interface: NonNull<*mut EfiVoid>,
) -> EfiStatus;

// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-installmultipleprotocolinterfaces
pub type EfiInstallMultipleProtocolInterfaces =
    extern "efiapi" fn(handle: NonNull<EfiHandle>, ...) -> EfiStatus;

// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-uninstallmultipleprotocolinterfaces
pub type EfiUninstallMultipleProtocolInterfaces =
    extern "efiapi" fn(handle: EfiHandle, ...) -> EfiStatus;
