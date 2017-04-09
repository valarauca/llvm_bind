use super::llvm_sys::*;
use super::llvm_sys::prelude::*;
use super::llvm_sys::core::*;
use super::llvm_sys::analysis::*;
use super::llvm_sys::bit_reader::*;

use std::ffi::{CString,CStr};
use std::os::raw::c_char;
use std::io::prelude::*;
use std::fs::{OpenOptions,File};
use std::io;
use std::path::Path;

use super::Buffers;
use super::module::Module;

const NULLPTR: &'static str = "
Buffer name value has a null ptr
";

/// Abstraction around llvm::MemoryBuffer
///
/// All memory buffers have names. So you'll
/// have to provide one on creation. Most often
/// MemoryBuffer is related to a single 
/// file/class/object in Code. So that is
/// a decent naming convention (I think).
pub struct Buffer {
    data: LLVMMemoryBufferRef,
    buffers: Vec<Buffers>,
}
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe{
            //drop LLVM memory buffer
            LLVMDisposeMemoryBuffer(self.data);
        }
    }
}
impl Buffer {

    /// Create a new memorybuffer of a specified
    /// size in bytes.
    ///
    /// It should be noted there is no method to
    /// resize buffers. So you may want to make
    /// this larger then you expect.
    ///
    /// This will zero the buffer before giving
    /// it to the LLVM.
    pub fn with_capacity<S: Into<Vec<u8>>>(size: usize, name: S) -> Buffer {
        let mut v = Vec::<u8>::with_capacity(size);
        for _ in 0..size {
            v.push(0u8);
        }
        let name = CString::new(name).expect(NULLPTR);
        unsafe{
            let b_ptr = v.as_ptr() as *const c_char;
            let name_ptr = name.as_ptr() as *const c_char;
            let buf = LLVMCreateMemoryBufferWithMemoryRange(b_ptr, size, name_ptr, 0);
            Buffer {
                data: buf,
                buffers: vec![Buffers::A(name), Buffers::B(v)]
            }
        }
    }

    /// Create a new memory buffer by copying
    /// from another buffer.
    pub fn copy_from<S: Into<Vec<u8>>>(buf: &[u8], name: S) -> Buffer {
        let name = CString::new(name).expect(NULLPTR);
        unsafe{
            let len = buf.len();
            let ptr = buf.as_ptr() as *const c_char;
            let name_ptr = name.as_ptr() as *const c_char;
            let buf = LLVMCreateMemoryBufferWithMemoryRangeCopy(ptr, len, name_ptr);
            Buffer {
                data: buf,
                buffers: vec![Buffers::A(name)]
            }
        }
    }
  

    /// Parse IR
    ///
    /// If this item contains LLVM-IR this function will
    /// attempt to parse it and convert it into
    /// an LLVM Module
    pub fn parse_ir(self) -> Result<Module,(Buffer,CString)> {
        use std::mem;
        
        let mut s = self;
        let mut modu = Module::new(s.get_name().to_bytes());
        let mut v = Vec::<Buffers>::with_capacity(0);
        mem::swap(&mut v, &mut s.buffers);
        unsafe {
            let mut err: *mut c_char = mem::zeroed();
            let flag = LLVMParseBitcode(s.data,modu.raw_module(),&mut err);
            if flag != 0 {
                mem::swap(&mut v, &mut s.buffers);
                Err((s,CString::from_raw(err)))
            } else {
                modu.append_buffers(&mut v);
                Ok(modu)
            }
        }
    } 

    /// From Rust Buffer
    ///
    /// You provide this interface with a buffer you own
    /// and it will expose that buffer to the LLVM
    /// as a MemBuffer
    ///
    /// This provides a zero copy way to load data
    pub fn from_owned<S: Into<Vec<u8>>>(buf: Vec<u8>, name: S) -> Buffer {
        let name = CString::new(name).expect(NULLPTR);
        unsafe{
            let len = buf.len();
            let ptr = buf.as_ptr() as *const c_char;
            let n_ptr = name.as_ptr() as *const c_char;
            let llvm = LLVMCreateMemoryBufferWithMemoryRange(ptr, len, n_ptr, 0);
            Buffer {
                data: llvm,
                buffers: vec![Buffers::A(name),Buffers::B(buf)]
            }
        }
    }

    /// Read From File
    ///
    /// Reads a memory buffer from File.
    /// No name is required as the name of the
    /// file is used as it's name.
    ///
    /// #Panic:
    ///
    /// This function will panic if path has
    /// no name
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Buffer>  {
        let name = path
            .as_ref()
            .file_name()
            .expect("Path has no file")
            .to_os_string()
            .into_string()
            .expect("Path isn't utf-8 parsable");
        let mut f: File = OpenOptions::new()
            .write(false)
            .create(false)
            .read(true)
            .open(path)?;
        let size = f.metadata()?.len() as usize;
        let mut buff = Vec::with_capacity(size);
        let _ = f.read_to_end(&mut buff)?;
        Ok(Buffer::from_owned(buff,name))
    }

    /// Get the name of this item
    pub fn get_name<'a>(&self) -> &'a CStr {
        use std::mem::transmute;
        
        unsafe {
            let ptr = LLVMGetValueName(transmute(self.data));
            let val: usize = transmute(ptr);
            if val == 0 {
                panic!("Module Name returned a null ptr");
            }
            CStr::from_ptr(ptr)
        }
    }

    /// From raw
    ///
    /// unsafely buids this item from it's raw components. Primarily
    /// used for internal interfaces
    pub unsafe fn from_raw(x: LLVMMemoryBufferRef, buf: Vec<Buffers>) -> Buffer {
        Buffer {
            data: x,
            buffers: buf
        }
    }
 
}

