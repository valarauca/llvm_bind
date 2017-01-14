
use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::analysis::*;
use llvm_sys::bit_writer::*;

use std::ffi::{CString,CStr};
use std::os::raw::c_char;


use super::Buffers;
use super::buffer::Buffer;
use super::target::get_local_triple;

const NULLPTR: &'static str = "
Module name has a null ptr
";

/// Abstruction around llvm:Module
///
/// Module is a unit of code compilation
pub struct Module {
  data: LLVMModuleRef,
  buffers: Vec<Buffers>
}
impl Drop for Module {
  fn drop(&mut self) {
    unsafe{
      LLVMDisposeModule(self.data);
    }
  }
}
impl Module {

  ///Create new Module with a name
  pub fn new<S>(name: S)
  -> Self 
  where S: Into<Vec<u8>> {
    let name = CString::new(name).expect(NULLPTR);
    unsafe {
      let n_ptr = name.as_ptr() as *const c_char;
      let m = LLVMModuleCreateWithName(n_ptr);
      Module {
        data: m,
        buffers: vec![Buffers::A(name)]
      }
    }
  }

  /// Verify Module Contents
  pub fn verify(&self) 
  -> Result<(), CString> {
    use std::mem;
        
    unsafe{
      let mut err : *mut c_char = mem::zeroed();
      let flag: i32 = LLVMVerifyModule(
          self.data, 
          LLVMVerifierFailureAction::LLVMReturnStatusAction, 
          &mut err
      );
      if flag != 0 {
          Err(CString::from_raw(err))
      } else {
          Ok(())
      }
    }
  }

  /// Write Module to a Buffer
  /// as IR
  pub fn to_ir(mut self) -> Buffer {
    use std::mem;
    
    let mut v = Vec::<Buffers>::with_capacity(0);
    mem::swap(&mut v, &mut self.buffers);
    unsafe{
      let buf = LLVMWriteBitcodeToMemoryBuffer(
          self.data
      );
      Buffer::from_raw(buf,v)
    }
  } 
  
  /// Set Target Triple
  ///
  /// Set what type of machine/os this module is being
  /// compiled too
  pub fn set_target(&mut self, triple: &str) {
    let buf = CString::new(triple).expect(NULLPTR);
    unsafe{  
      LLVMSetTarget(self.data, buf.as_ptr());
    }
    self.buffers.push(Buffers::A(buf));
  }

  /// Set _This_ triple
  ///
  /// Set the model to the default target
  /// triple for the system the code is
  /// currently executing on
  pub fn set_default_triple(&mut self) {
    let buf = get_local_triple();
    unsafe {
      LLVMSetTarget(self.data, buf.as_ptr());
    }
    self.buffers.push(Buffers::A(buf));
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

  /// From Raw
  ///
  /// Unsafely construct this object
  pub fn from_raw(x: LLVMModuleRef, buffers: Vec<Buffers>)
  -> Self {
    Module {
      data: x,
      buffers: buffers
    }
  } 

  /// Append Buffers
  ///
  /// If a module takes ownership of new buffers
  /// it needs to adds them to it's internal vector
  /// this function does nothing unsafe internally.
  ///
  /// But if you are calling it, it is because you
  /// are likely doing something wildy unsfae before
  /// this.
  pub unsafe fn append_buffers(&mut self, b: &mut Vec<Buffers>) {
    self.buffers.append(b);
  }

  /// Raw Module
  ///
  /// Returns a raw pointer to the underlying data type
  pub unsafe fn raw_module(&mut self) -> *mut LLVMModuleRef {
    use std::mem::transmute;    
    transmute(&mut self.data)
  }
}

