use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::ProcessStatus::*;
use windows_sys::Win32::System::Threading::*;

use crate::error::{mm_error, Result};
use crate::module::ModuleInfo;
use crate::ntapi::PROCESS_ALL_ACCESS_MEMORY;

#[derive(Debug)]
pub struct p_handle {
    handle: HANDLE,
    pid: u32,
}

impl p_handle {
    pub fn open_by_pid_with_access(pid: u32, access: u32) -> Result<Self> {
        let handle = unsafe { OpenProcess(access, 0, pid) };
        if handle == std::ptr::null_mut()  || handle == INVALID_HANDLE_VALUE {
            return Err(mm_error::ProcessOpenFailed(format!(
                "debug-pid -> {} with access 0x{:08X}",
                pid, access
            )));
        }
        Ok(Self { handle, pid })
    }

    pub fn open_by_pid(pid: u32) -> Result<Self> {
        Self::open_by_pid_with_access(pid, PROCESS_ALL_ACCESS_MEMORY)
    }

    fn get_process_name_from_handle(handle: HANDLE) -> Option<String> {
        let mut module: HMODULE = std::ptr::null_mut();
        let mut needed: u32 = 0;

        let success = unsafe {
            EnumProcessModulesEx(
                handle,
                &mut module,
                std::mem::size_of::<HMODULE>() as u32,
                &mut needed,
                LIST_MODULES_ALL,
            )
        };

        if success == 0 {
            return None;
        }

        let mut name_buf: [u16; 260] = [0; 260];
        let len = unsafe { GetModuleBaseNameW(handle, module, name_buf.as_mut_ptr(), 260) };
        if len == 0 {
            return None;
        }

        let name = OsString::from_wide(&name_buf[..len as usize]);
        name.into_string().ok()
    }

    pub fn find_process_by_name(name: &str) -> Result<u32> {
        let mut pids: [u32; 2048] = [0; 2048];
        let mut bytes_ret: u32 = 0;

        let success = unsafe {
            EnumProcesses(
                pids.as_mut_ptr(),
                std::mem::size_of_val(&pids) as u32,
                &mut bytes_ret,
            )
        };

        if success == 0 {
            return Err(mm_error::ProcessEnumFailed(unsafe { GetLastError() } as i32));
        }

        let process_count = bytes_ret as usize / std::mem::size_of::<u32>();
        let target_name = name.to_lowercase();

        for &pid in pids.iter().take(process_count) {
            if pid == 0 {
                continue;
            }

            let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };

            if handle == std::ptr::null_mut() || handle == INVALID_HANDLE_VALUE {
                continue;
            }

            let process_name = Self::get_process_name_from_handle(handle);

            unsafe { CloseHandle(handle) };

            if let Some(proc_name) = process_name {
                if proc_name.to_lowercase() == target_name {
                    return Ok(pid);
                }
            }
        }

        Err(mm_error::ProcessNotFound(name.to_string()))
    }

    #[must_use]
    #[inline]
    pub const fn as_raw(&self) -> HANDLE {
        self.handle
    }

    #[must_use]
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.handle != std::ptr::null_mut() && self.handle != INVALID_HANDLE_VALUE
    }

    pub fn enumerate_modules(&self) -> Result<Vec<ModuleInfo>> {
        const MAX_MODULES: usize = 1024;
        let mut modules: [HMODULE; MAX_MODULES] = [std::ptr::null_mut(); MAX_MODULES];
        let mut needed: u32 = 0;

        let success = unsafe {
            EnumProcessModulesEx(
                self.handle,
                modules.as_mut_ptr(),
                std::mem::size_of_val(&modules) as u32,
                &mut needed,
                LIST_MODULES_ALL,
            )
        };

        if success == 0 {
            let error = unsafe { GetLastError() };
            return Err(mm_error::ModuleEnumFailed(error));
        }

        let module_count = needed as usize / std::mem::size_of::<HMODULE>();
        let mut result = Vec::with_capacity(module_count);

        for &module in modules.iter().take(module_count) {
            if module == std::ptr::null_mut() {
                continue;
            }

            let mut name_buf: [u16; 260] = [0; 260];
            let name_len = unsafe { GetModuleBaseNameW(self.handle, module, name_buf.as_mut_ptr(), 260) };

            if name_len == 0 {
                continue;
            }

            let name = OsString::from_wide(&name_buf[..name_len as usize])
                .into_string()
                .unwrap_or_default();

            let mut info: MODULEINFO = unsafe { std::mem::zeroed() };

            let i_success = unsafe {
                GetModuleInformation(
                    self.handle,
                    module,
                    &mut info,
                    std::mem::size_of::<MODULEINFO>() as u32,
                )
            };

            if i_success == 0 {
                continue;
            }

            result.push(ModuleInfo {
                name,
                addy: info.lpBaseOfDll as usize,
                size: info.SizeOfImage as usize,
                entry_point: info.EntryPoint as usize,
            });
        }

        Ok(result)
    }

    pub fn get_module_base(&self, module_name: &str) -> Result<usize> {
        let modules = self.enumerate_modules()?;
        let target_name = module_name.to_lowercase();

        modules
            .iter()
            .find(|m| m.name.to_lowercase() == target_name)
            .map(|m| m.addy)
            .ok_or_else(|| mm_error::ModuleNotFound(module_name.to_string()))
    }
}

impl Drop for p_handle {
    fn drop(&mut self) {
        if self.is_valid() {
            unsafe { CloseHandle(self.handle) };
        }
    }
}

unsafe impl Send for p_handle {}
unsafe impl Sync for p_handle {}