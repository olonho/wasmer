//! The middleware parses the function binary bytecodes and transform them
//! with the chosen functions.
use wasmparser::{BinaryReader, Operator, Result, Type};

/// A Middleware binary reader of the WebAssembly structures and types.
#[derive(Clone, Debug)]
pub struct MiddlewareBinaryReader<'a> {
    inner: BinaryReader<'a>,
}

impl<'a> MiddlewareBinaryReader<'a> {
    /// Constructs a `MiddlewareBinaryReader` with an explicit starting offset.
    pub fn new_with_offset(data: &'a [u8], original_offset: usize) -> Self {
        let inner = BinaryReader::new_with_offset(data, original_offset);
        Self { inner }
    }

    /// Read a `count` indicating the number of times to call `read_local_decl`.
    pub fn read_local_count(&mut self) -> Result<usize> {
        self.inner.read_local_count()
    }

    /// Read a `(count, value_type)` declaration of local variables of the same type.
    pub fn read_local_decl(&mut self, locals_total: &mut usize) -> Result<(u32, Type)> {
        self.inner.read_local_decl(locals_total)
    }

    /// Reads the next available `Operator`.
    pub fn read_operator(&mut self) -> Result<Operator<'a>> {
        self.inner.read_operator()
    }

    /// Returns the inner `BinaryReader`'s current position.
    pub fn current_position(&self) -> usize {
        self.inner.current_position()
    }

    /// Returns the inner `BinaryReader`'s original position (with the offset)
    pub fn original_position(&self) -> usize {
        self.inner.original_position()
    }

    /// Returns the number of bytes remaining in the inner `BinaryReader`.
    pub fn bytes_remaining(&self) -> usize {
        self.inner.bytes_remaining()
    }

    /// Returns whether the inner `BinaryReader` has reached the end of the file.
    pub fn eof(&self) -> bool {
        self.inner.eof()
    }
}
