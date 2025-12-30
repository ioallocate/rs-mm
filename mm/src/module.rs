#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInfo {
    pub name: String,
    pub addy: usize,
    pub size: usize,
    pub entry_point: usize,
}

impl ModuleInfo {
    #[must_use]
    pub const fn new(name: String, addy: usize, size: usize, entry_point: usize) -> Self {
        Self {
            name,
            addy,
            size,
            entry_point,
        }
    }

    #[must_use]
    #[inline]
    pub const fn end_address(&self) -> usize {
        self.addy + self.size
    }

    #[must_use]
    #[inline]
    pub const fn contains_address(&self, address: usize) -> bool {
        address >= self.addy && address < self.end_address()
    }

    #[must_use]
    #[inline]
    pub const fn address_to_index(&self, address: usize) -> Option<usize> {
        if self.contains_address(address) {
            Some(address - self.addy)
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub const fn index_to_address(&self, index: usize) -> Option<usize> {
        if index < self.size {
            Some(self.addy + index)
        } else {
            None
        }
    }
}

impl std::fmt::Display for ModuleInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ 0x{:016X} (size: 0x{:X}, entry: 0x{:016X})",
            self.name, self.addy, self.size, self.entry_point
        )
    }
}