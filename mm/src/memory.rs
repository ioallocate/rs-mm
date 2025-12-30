use std::marker::PhantomData;

use crate::error::{mm_error, Result};
use crate::handle::p_handle;
use crate::ntapi::{nt_read_virtual_memory, nt_write_virtual_memory};

#[derive(Debug)]
pub struct mmg<'a> {
    p_handle: &'a p_handle,
}

impl<'a> mmg<'a> {
    #[must_use]
    #[inline]
    pub const fn new(handle: &'a p_handle) -> Self {
        Self { p_handle: handle }
    }

    #[inline]
    pub fn read<T>(&self, address: usize) -> Result<T>
    where
        T: Copy + Default,
    {
        let mut value = T::default();
        let size = std::mem::size_of::<T>();

        let bytes_read = unsafe {
            nt_read_virtual_memory(
                self.p_handle.as_raw(),
                address,
                std::ptr::addr_of_mut!(value).cast::<u8>(),
                size,
            )
                .map_err(|status| mm_error::ReadFailed { address, status })?
        };

        if bytes_read != size {
            return Err(mm_error::InvalidBufferSize {
                expected: size,
                actual: bytes_read,
            });
        }

        Ok(value)
    }

    #[inline]
    pub fn write<T>(&self, address: usize, value: &T) -> Result<usize>
    where
        T: Copy,
    {
        let size = std::mem::size_of::<T>();

        unsafe {
            nt_write_virtual_memory(
                self.p_handle.as_raw(),
                address,
                std::ptr::addr_of!(*value).cast::<u8>(),
                size,
            )
                .map_err(|status| mm_error::WriteFailed { address, status })
        }
    }

    #[inline]
    pub fn read_bytes(&self, address: usize, buffer: &mut [u8]) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        unsafe {
            nt_read_virtual_memory(
                self.p_handle.as_raw(),
                address,
                buffer.as_mut_ptr(),
                buffer.len(),
            )
                .map_err(|status| mm_error::ReadFailed { address, status })
        }
    }

    #[inline]
    pub fn write_bytes(&self, address: usize, buffer: &[u8]) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        unsafe {
            nt_write_virtual_memory(self.p_handle.as_raw(), address, buffer.as_ptr(), buffer.len())
                .map_err(|status| mm_error::WriteFailed { address, status })
        }
    }

    pub fn read_bytes_vec(&self, address: usize, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        let bytes_read = self.read_bytes(address, &mut buffer)?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    pub fn read_string(&self, address: usize, max_length: usize) -> Result<String> {
        let buffer = self.read_bytes_vec(address, max_length)?;
        let null_pos = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());

        String::from_utf8(buffer[..null_pos].to_vec()).map_err(|_| mm_error::InvalidBufferSize {
            expected: max_length,
            actual: null_pos,
        })
    }

    pub fn write_string(&self, address: usize, string: &str) -> Result<usize> {
        let mut bytes = string.as_bytes().to_vec();
        bytes.push(0);
        self.write_bytes(address, &bytes)
    }

    pub fn read_wstring(&self, address: usize, max_chars: usize) -> Result<String> {
        let byte_count = max_chars * 2;
        let buffer = self.read_bytes_vec(address, byte_count)?;

        let u16_buffer: Vec<u16> = buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        let null_pos = u16_buffer.iter().position(|&c| c == 0).unwrap_or(u16_buffer.len());

        String::from_utf16(&u16_buffer[..null_pos]).map_err(|_| mm_error::InvalidBufferSize {
            expected: max_chars,
            actual: null_pos,
        })
    }

    pub fn write_wstring(&self, address: usize, string: &str) -> Result<usize> {
        let mut u16_buffer: Vec<u16> = string.encode_utf16().collect();
        u16_buffer.push(0);

        let bytes: Vec<u8> = u16_buffer.iter().flat_map(|&c| c.to_le_bytes()).collect();

        self.write_bytes(address, &bytes)
    }

    pub fn read_pointer_chain(&self, base: usize, offsets: &[usize]) -> Result<usize> {
        let mut address = base;

        for (index, &offset) in offsets.iter().enumerate() {
            address = address.wrapping_add(offset);

            if index < offsets.len() - 1 {
                address = self.read::<usize>(address)?;
            }
        }

        Ok(address)
    }

    #[must_use]
    #[inline]
    pub const fn typed<T: Copy + Default>(&self) -> TypeReader<'a, T> {
        TypeReader {
            handle: self.p_handle,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct TypeReader<'a, T> {
    handle: &'a p_handle,
    _phantom: PhantomData<T>,
}

impl<'a, T: Copy + Default> TypeReader<'a, T> {
    #[inline]
    pub fn read(&self, address: usize) -> Result<T> {
        mmg::new(self.handle).read(address)
    }

    #[inline]
    pub fn write(&self, address: usize, value: &T) -> Result<usize> {
        mmg::new(self.handle).write(address, value)
    }

    pub fn read_array(&self, address: usize, count: usize) -> Result<Vec<T>> {
        let reader = mmg::new(self.handle);
        let size = std::mem::size_of::<T>();
        let mut result = Vec::with_capacity(count);

        for i in 0..count {
            let value = reader.read::<T>(address + i * size)?;
            result.push(value);
        }

        Ok(result)
    }

    pub fn write_array(&self, address: usize, values: &[T]) -> Result<usize> {
        let reader = mmg::new(self.handle);
        let size = std::mem::size_of::<T>();
        let mut total_written = 0;

        for (i, value) in values.iter().enumerate() {
            total_written += reader.write(address + i * size, value)?;
        }

        Ok(total_written)
    }
}