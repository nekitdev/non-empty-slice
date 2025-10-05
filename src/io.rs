#[cfg(not(feature = "std"))]
compile_error!("expected `std` to be enabled");

use core::fmt;

use std::io::{IoSlice, Result, Write};

use crate::{slice::NonEmptyBytes, vec::NonEmptyByteVec};

type Bytes = [u8];
type ByteSlices<'a> = [IoSlice<'a>];

impl Write for &mut NonEmptyBytes {
    fn write(&mut self, buffer: &Bytes) -> Result<usize> {
        self.as_mut_slice().write(buffer)
    }

    fn write_vectored(&mut self, buffers: &ByteSlices<'_>) -> Result<usize> {
        self.as_mut_slice().write_vectored(buffers)
    }

    fn write_all(&mut self, buffer: &Bytes) -> Result<()> {
        self.as_mut_slice().write_all(buffer)
    }

    fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> Result<()> {
        self.as_mut_slice().write_fmt(arguments)
    }

    fn flush(&mut self) -> Result<()> {
        self.as_mut_slice().flush()
    }
}

impl Write for NonEmptyByteVec {
    fn write(&mut self, buffer: &Bytes) -> Result<usize> {
        // SAFETY: writing can not make the vector empty
        unsafe { self.as_mut_vec().write(buffer) }
    }

    fn write_vectored(&mut self, buffers: &ByteSlices<'_>) -> Result<usize> {
        // SAFETY: writing can not make the vector empty
        unsafe { self.as_mut_vec().write_vectored(buffers) }
    }

    fn write_all(&mut self, buffer: &Bytes) -> Result<()> {
        // SAFETY: writing can not make the vector empty
        unsafe { self.as_mut_vec().write_all(buffer) }
    }

    fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> Result<()> {
        // SAFETY: writing can not make the vector empty
        unsafe { self.as_mut_vec().write_fmt(arguments) }
    }

    fn flush(&mut self) -> Result<()> {
        // SAFETY: flushing can not make the vector empty
        unsafe { self.as_mut_vec().flush() }
    }
}
