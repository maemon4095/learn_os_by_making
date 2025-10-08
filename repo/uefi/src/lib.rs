#![no_std]
#![no_main]
#![feature(offset_of)]
#![feature(extern_types)]
#![feature(extended_varargs_abi_support)]

mod efi_configuration_table;
mod efi_revision;
mod efi_runtime_services;
mod efi_system_table;
mod efi_table_header;

pub mod data_type;
pub mod efi_boot_services;
pub mod protocol;

pub use data_type::*;
pub use efi_boot_services::EfiBootServices;
pub use efi_revision::EfiRevision;
pub use efi_runtime_services::EfiRuntimeServices;
pub use efi_table_header::EfiTableHeader;

pub type EfiResult<T> = core::result::Result<T, &'static str>;
