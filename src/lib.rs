//! Traits for handling bytes data from external memory.
//!
//! Currently only read functionality is supported.
#![no_std]
#![deny(unused_crate_dependencies)]

#[cfg(not(feature = "std"))]
extern crate core;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(feature = "std")]
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    string::String,
};

/// External addressable memory.
pub trait ExternalMemory: Debug {
    /// Errors specific to memory accessing.
    type ExternalMemoryError: Debug + Display + Eq + PartialEq;
}

/// `ExternalMemory` could also be applied to regular RAM.
impl ExternalMemory for () {
    type ExternalMemoryError = NoEntries;
}

/// Empty error enum, for cases with fault-free memory access.
#[derive(Debug, Eq, PartialEq)]
pub enum NoEntries {}

impl Display for NoEntries {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "")
    }
}

/// Bytes access through [`ExternalMemory`].
///
/// Could be implemented, for example, for a combination of an address in
/// external memory and corresponding bytes slice length.
pub trait AddressableBuffer<E: ExternalMemory>: Sized {
    /// Bytes read from buffer.
    type ReadBuffer: AsRef<[u8]>;

    /// Total length of the addressable buffer.
    fn total_len(&self) -> usize;

    /// Read bytes slice of known length at known relative position.
    ///
    /// Important to keep `read_slice`, **not `read_byte`** as a basic reader
    /// tool, because of commonly occuring pages in memory.
    fn read_slice(
        &self,
        ext_memory: &mut E,
        position: usize,
        slice_len: usize,
    ) -> Result<Self::ReadBuffer, BufferError<E>>;

    /// Read single byte at known position.
    fn read_byte(&self, ext_memory: &mut E, position: usize) -> Result<u8, BufferError<E>> {
        let byte_slice = self.read_slice(ext_memory, position, 1)?;
        Ok(byte_slice.as_ref()[0])
    }

    /// Restrict the length of the addressable buffer.
    fn limit_length(&self, new_len: usize) -> Result<Self, BufferError<E>>;
}

/// `AddressableBuffer` could be also implemented for regular bytes slices.
impl<'a, E: ExternalMemory> AddressableBuffer<E> for &'a [u8] {
    type ReadBuffer = &'a [u8];
    fn total_len(&self) -> usize {
        self.len()
    }
    fn read_slice(
        &self,
        _ext_memory: &mut E,
        position: usize,
        slice_len: usize,
    ) -> Result<Self::ReadBuffer, BufferError<E>> {
        if self.len() < position {
            return Err(BufferError::OutOfRange {
                position,
                total_length: self.len(),
            });
        }
        match self.get(position..position + slice_len) {
            Some(a) => Ok(a),
            None => Err(BufferError::DataTooShort {
                position,
                minimal_length: slice_len,
            }),
        }
    }
    fn limit_length(&self, new_len: usize) -> Result<Self, BufferError<E>> {
        self.get(..new_len).ok_or(BufferError::DataTooShort {
            position: 0,
            minimal_length: new_len,
        })
    }
}

/// Errors in buffer access.
#[derive(Debug, Eq, PartialEq)]
pub enum BufferError<E: ExternalMemory> {
    DataTooShort {
        position: usize,
        minimal_length: usize,
    },
    External(E::ExternalMemoryError),
    OutOfRange {
        position: usize,
        total_length: usize,
    },
}

impl<E: ExternalMemory> BufferError<E> {
    fn error_text(&self) -> String {
        match &self {
            BufferError::DataTooShort { position, minimal_length } => format!("Data is too short for expected content. Expected at least {minimal_length} element(s) after position {position}."),
            BufferError::External(e) => format!("Error accessing external memory. {e}"),
            BufferError::OutOfRange { position, total_length } => format!("Position {position} is out of range for data length {total_length}."),
        }
    }
}

impl<E: ExternalMemory> Display for BufferError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.error_text())
    }
}

#[cfg(feature = "std")]
impl<E: ExternalMemory> Error for BufferError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
