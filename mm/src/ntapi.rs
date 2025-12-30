use windows_sys::Win32::Foundation::{HANDLE, NTSTATUS};

pub const STATUS_SUCCESS: NTSTATUS = 0x00;
pub const STATUS_PARTIAL_COPY: NTSTATUS = 0x8000_000D_u32 as i32;

pub const PROCESS_VM_READ: u32 = 0x0010;
pub const PROCESS_VM_WRITE: u32 = 0x0020;
pub const PROCESS_VM_OPERATION: u32 = 0x0008;
pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;

pub const PROCESS_ALL_ACCESS_MEMORY: u32 =
    PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_QUERY_INFORMATION;

#[link(name = "ntdll")]
unsafe extern "system" {
    pub fn NtReadVirtualMemory(
        process_handle: HANDLE,
        base_address: *const core::ffi::c_void,
        buffer: *mut core::ffi::c_void,
        buffer_size: usize,
        bytes_read: *mut usize,
    ) -> NTSTATUS;

    pub fn NtWriteVirtualMemory(
        process_handle: HANDLE,
        base_address: *mut core::ffi::c_void,
        buffer: *const core::ffi::c_void,
        buffer_size: usize,
        bytes_written: *mut usize,
    ) -> NTSTATUS;
}

#[inline]
pub unsafe fn nt_read_virtual_memory(
    process_handle: HANDLE,
    address: usize,
    buffer: *mut u8,
    size: usize,
) -> Result<usize, NTSTATUS> {
    let mut bytes_read: usize = 0;

    let status = NtReadVirtualMemory(
        process_handle,
        address as *const core::ffi::c_void,
        buffer.cast::<core::ffi::c_void>(),
        size,
        &mut bytes_read,
    );

    if status == STATUS_SUCCESS {
        Ok(bytes_read)
    } else {
        Err(status)
    }
}

#[inline]
pub unsafe fn nt_write_virtual_memory(
    process_handle: HANDLE,
    address: usize,
    buffer: *const u8,
    size: usize,
) -> Result<usize, NTSTATUS> {
    let mut bytes_written: usize = 0;

    let status = NtWriteVirtualMemory(
        process_handle,
        address as *mut core::ffi::c_void,
        buffer.cast::<core::ffi::c_void>(),
        size,
        &mut bytes_written,
    );

    if status == STATUS_SUCCESS {
        Ok(bytes_written)
    } else {
        Err(status)
    }
}

#[inline]
#[must_use]
pub const fn nt_success(status: NTSTATUS) -> bool {
    status >= 0
}

#[inline]
#[must_use]
pub const fn nt_information(status: NTSTATUS) -> bool {
    (status as u32 >> 30) == 1
}

#[inline]
#[must_use]
pub const fn nt_warning(status: NTSTATUS) -> bool {
    (status as u32 >> 30) == 2
}

#[inline]
#[must_use]
pub const fn nt_error(status: NTSTATUS) -> bool {
    (status as u32 >> 30) == 3
}