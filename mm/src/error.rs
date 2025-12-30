use thiserror::Error;

pub type Result<T> = std::result::Result<T, mm_error>;

#[derive(Error, Debug)]
pub enum mm_error {
    #[error("mm_error -> failed to open process: {0}")]
    ProcessOpenFailed(String),

    #[error("mm_error -> failed to find specific process {0}")]
    ProcessNotFound(String),

    #[error("mm_error -> failed to enumerate processes. ntstatus -> 0x{0:08X}")]
    ProcessEnumFailed(i32),

    #[error("mm_error -> module not found {0}")]
    ModuleNotFound(String),

    #[error("mm_error -> module enumeration failed, ec . {0}")]
    ModuleEnumFailed(u32),

    #[error("mm_error -> mem read failed at -> 0x{address:016X}: ntstatus 0x{status:08X}")]
    ReadFailed { address: usize, status: i32 },

    #[error("mm_error -> mem write failed at -> 0x{address:016X}: ntstatus 0x{status:08X}")]
    WriteFailed { address: usize, status: i32 },

    #[error("mm_error -> Invalid buffer size : expected {expected}, got {actual}")]
    InvalidBufferSize { expected: usize, actual: usize },

    #[error("mm_error -> null ptr encountered")]
    NullPointer,

    #[error("mm_error -> invalid handle")]
    InvalidHandle,

    #[error("mm_error -> access denied, missing privilidges")]
    AccessDenied,

    #[error("mm_error -> region not accessable at 0x{0:016X}")]
    MemoryNotAccessable(usize),

    #[error("mm_error -> windows api error . {0}")]
    WindowsApiError(u32),
}

impl mm_error {
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(self, Self::ReadFailed { .. } | Self::WriteFailed { .. })
    }

    #[must_use]
    pub const fn ntstatus(&self) -> Option<i32> {
        match self {
            Self::ReadFailed { status, .. } | Self::WriteFailed { status, .. } => Some(*status),
            Self::ProcessEnumFailed(status) => Some(*status),
            _ => None,
        }
    }
}