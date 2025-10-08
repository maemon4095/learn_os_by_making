//! REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#memory-allocation-services

use core::ptr::NonNull;

use crate::{EfiStatus, EfiVoid};

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-allocatepages
pub type EfiAllocatePages = extern "efiapi" fn(
    allocate_type: EfiAllocateType,
    memory_type: EfiMemoryType,
    pages: usize,
    memory: NonNull<EfiPhysicalAddress>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-allocatepages
#[derive(Debug)]
#[repr(u32)]
pub enum EfiAllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-allocatepages
#[derive(Debug)]
#[repr(u32)]
pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-allocatepages
#[derive(Debug)]
#[repr(transparent)]
pub struct EfiPhysicalAddress(u64);

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-freepages
pub type EfiFreePages = extern "efiapi" fn(memory: EfiPhysicalAddress, pages: usize) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-getmemorymap
pub type EfiGetMemoryMap = extern "efiapi" fn(
    memory_map_size: NonNull<usize>,
    memory_map: NonNull<EfiMemoryDescriptor>,
    map_key: NonNull<usize>,
    descriptor_size: NonNull<usize>,
    descriptor_version: NonNull<u32>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-getmemorymap
#[derive(Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    memory_type: EfiMemoryType,
    physical_start: EfiPhysicalAddress,
    virtual_start: EfiVirtualAddress,
    number_of_pages: u64,
    attribute: EfiMemoryAttribute,
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-getmemorymap
#[derive(Debug)]
#[repr(transparent)]
pub struct EfiVirtualAddress(u64);

#[derive(Debug)]
#[repr(transparent)]
pub struct EfiMemoryAttribute(u64);

impl core::ops::BitOr for EfiMemoryAttribute {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl EfiMemoryAttribute {
    /// Memory cacheability attribute: The memory region supports being configured as not cacheable.
    pub const MEMORY_UC: Self = Self(0x0000000000000001);
    /// Memory cacheability attribute: The memory region supports being configured as write combining.
    pub const MEMORY_WC: Self = Self(0x0000000000000002);
    /// Memory cacheability attribute: The memory region supports being configured as cacheable with a “write through” policy. Writes that hit in the cache will also be written to main memory.
    pub const MEMORY_WT: Self = Self(0x0000000000000004);
    /// Memory cacheability attribute: The memory region supports being configured as cacheable with a “write back” policy. Reads and writes that hit in the cache do not propagate to main memory. Dirty data is written back to main memory when a new cache line is allocated.
    pub const MEMORY_WB: Self = Self(0x0000000000000008);
    /// Memory cacheability attribute: The memory region supports being configured as not cacheable, exported, and supports the “fetch and add” semaphore mechanism.
    pub const MEMORY_UCE: Self = Self(0x0000000000000010);
    /// Physical memory protection attribute: The memory region supports being configured as write-protected by system hardware. This is typically used as a cacheability attribute today. The memory region supports being configured as cacheable with a “write protected” policy. Reads come from cache lines when possible, and read misses cause cache fills. Writes are propagated to the system bus and cause corresponding cache lines on all processors on the bus to be invalidated.
    pub const MEMORY_WP: Self = Self(0x0000000000001000);
    /// Physical memory protection attribute: The memory region supports being configured as read-protected by system hardware.
    pub const MEMORY_RP: Self = Self(0x0000000000002000);
    /// Physical memory protection attribute: The memory region supports being configured so it is protected by system hardware from executing code.
    pub const MEMORY_XP: Self = Self(0x0000000000004000);
    /// Runtime memory attribute: The memory region refers to persistent memory
    pub const MEMORY_NV: Self = Self(0x0000000000008000);
    /// The memory region provides higher reliability relative to other memory in the system. If all memory has the same reliability, then this bit is not used.
    pub const MEMORY_MORE_RELIABLE: Self = Self(0x0000000000010000);
    /// Physical memory protection attribute: The memory region supports making this memory range read-only by system hardware.
    pub const MEMORY_RO: Self = Self(0x0000000000020000);
    /// Specific-purpose memory (SPM). The memory is earmarked for specific purposes such as for specific device drivers or applications. The SPM attribute serves as a hint to the OS to avoid allocating this memory for core OS data or code that can not be relocated. Prolonged use of this memory for purposes other than the intended purpose may result in suboptimal platform performance.
    pub const MEMORY_SP: Self = Self(0x0000000000040000);
    /// If this flag is set, the memory region is capable of being protected with the CPU’s memory cryptographic capabilities. If this flag is clear, the memory region is not capable of being protected with the CPU’s memory cryptographic capabilities or the CPU does not support CPU memory cryptographic capabilities.
    pub const MEMORY_CPU_CRYPTO: Self = Self(0x0000000000080000);
    /// Runtime memory attribute: The memory region needs to be given a virtual mapping by the operating system when SetVirtualAddressMap() is called (described in Virtual Memory Services.
    pub const MEMORY_RUNTIME: Self = Self(0x8000000000000000);
    /// If this flag is set, the memory region is described with additional ISA-specific memory attributes as specified in EFI_MEMORY_ISA_MASK .
    pub const MEMORY_ISA_VALID: Self = Self(0x4000000000000000);
    /// Defines the bits reserved for describing optional ISA-specific cacheability attributes that are not covered by the standard UEFI Memory Attributes cacheability bits (EFI_MEMORY_UC, EFI_MEMORY_WC, EFI_MEMORY_WT, EFI_MEMORY_WB and EFI_MEMORY_UCE). See Calling Conventions for further ISA-specific enumeration of these bits.
    pub const MEMORY_ISA_MASK: Self = Self(0x0FFFF00000000000);
}

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-allocatepool
pub type EfiAllocatePool = extern "efiapi" fn(
    pool_type: EfiMemoryType,
    size: usize,
    buffer: NonNull<*mut EfiVoid>,
) -> EfiStatus;

/// REF: https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html?highlight=efi_raise_tpl#efi-boot-services-freepool
pub type EfiFreePool = extern "efiapi" fn(buffer: NonNull<EfiVoid>) -> EfiStatus;
