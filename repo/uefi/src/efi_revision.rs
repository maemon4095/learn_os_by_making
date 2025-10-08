// REF: https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id4
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EfiRevision {
    value: u32,
}

impl From<u32> for EfiRevision {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl Into<u32> for EfiRevision {
    fn into(self) -> u32 {
        self.value
    }
}

impl EfiRevision {
    pub fn major(&self) -> u16 {
        (self.value >> 16) as u16
    }

    pub fn minor(&self) -> u16 {
        (self.value & 0xFFFF) as u16
    }

    pub fn minor_upper(&self) -> u16 {
        self.minor() / 10
    }

    pub fn minor_lower(&self) -> u16 {
        self.minor() % 10
    }
}

impl core::fmt::Display for EfiRevision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let major = self.major();
        let minor_upper = self.minor_upper();
        let minor_lower = self.minor_lower();

        if minor_lower == 0 {
            core::write!(f, "{}.{}", major, minor_upper)
        } else {
            core::write!(f, "{}.{}.{}", major, minor_upper, minor_lower)
        }
    }
}
