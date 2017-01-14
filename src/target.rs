

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::target_machine::*;

use super::Buffers;

use std::ffi::{CString,CStr};
use std::os::raw::c_char;
use std::default::Default;
use std::ptr::null_mut;

const NULLPTR: &'static str = "
Null pointer recieved by API
";

/// Abstraction around llvm::TargetMachine
///
/// This stores information related to the
/// target triple and the features it
/// contains
///
/// The C API exposes no way to drop this
/// value, so IDFK. Normally this should
/// just be passed to `TargetMachine` or
/// `TargetMachineBuilder`
pub struct Target {
  data: LLVMTargetRef,
  buffers: Vec<Buffers>
}
impl Target {
  
  /// With the name of a target
  pub fn from_name(name: &str) -> Target {
    unsafe {
        let cstr = CString::new(name).expect(NULLPTR);
        let tpr = LLVMGetTargetFromName(cstr.as_ptr());
        Target {
          data: tpr,
          buffers: vec![Buffers::A(cstr)]
        }
    }
  }
  pub fn get_description<'a>(&self) -> &'a CStr {
    unsafe {
      let ptr = LLVMGetTargetDescription(self.data);
      CStr::from_ptr(ptr)
    }
  }
  pub fn get_name<'a>(&self) -> &'a CStr {
    unsafe {
      let ptr = LLVMGetTargetName(self.data);
      CStr::from_ptr(ptr)
    }
  }
  pub fn has_asm_backend(&self) -> bool {
    unsafe {
      LLVMTargetHasAsmBackend(self.data) == 1
    }
  }
  pub fn has_target_machine(&self) -> bool {
    unsafe {
      LLVMTargetHasTargetMachine(self.data) == 1
    }
  }
  /// Internal Method used for building Targets
  pub unsafe fn split(self) -> (LLVMTargetRef, Vec<Buffers>) {
    (self.data, self.buffers)
  }
}
impl Default for Target {
  fn default() -> Self {
    unsafe{
      Target {
        data: LLVMGetFirstTarget(),
        buffers: Vec::with_capacity(0)
      }
    }
  }
}

/// Get target triple for _this_ target
///
/// This returns the target triple this library is
/// running on. Or thinks it is running on if you
/// cross compiled.
pub fn get_local_triple() -> CString {
  unsafe {
    let a = LLVMGetDefaultTargetTriple();
    let trip = CStr::from_ptr(a as *const _).to_owned();
    LLVMDisposeMessage(a);
    trip
  }
}

/// Set the Target Machine's Optimization level
///
/// Describes how aggressive optmization should be
#[derive(Copy,Clone,Debug)]
pub enum CodeGenOptLevel {
  None,
  Less,
  Default,
  Aggressive
}
impl Into<LLVMCodeGenOptLevel> for CodeGenOptLevel {
  /// Used within the library for operating on LLVM
  /// interfaces
  fn into(self) -> LLVMCodeGenOptLevel {
    match self {
      CodeGenOptLevel::None => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
      CodeGenOptLevel::Less => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
      CodeGenOptLevel::Default => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
      CodeGenOptLevel::Aggressive => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive
    }
  }
}
impl Default for CodeGenOptLevel {
  /// Returns the Enum that is labeled as
  /// `default` by the llvm source so
  /// I'm assuming that is a good
  /// default.
  fn default() -> Self {
    CodeGenOptLevel::Default
  }
}

/// Set the relocation mode
///
/// I have no clue what these do,
/// and the LLVM has next to no doc.
#[derive(Copy,Clone,Debug)]
pub enum RelocMode {
  Default,
  Static,
  PIC,
  DynamicNoPic
}
impl Into<LLVMRelocMode> for RelocMode {
  fn into(self) -> LLVMRelocMode {
    match self {
      RelocMode::Default => LLVMRelocMode::LLVMRelocDefault,
      RelocMode::Static => LLVMRelocMode::LLVMRelocStatic,
      RelocMode::PIC => LLVMRelocMode::LLVMRelocPIC,
      RelocMode::DynamicNoPic => LLVMRelocMode::LLVMRelocDynamicNoPic
    }
  }
}
impl Default for RelocMode {
  /// Returns the value labeled default.  
  fn default() -> Self {
    RelocMode::Default
  }
}

/// Code Model
///
/// This gives some hint on how the LLVM should
/// optimize code. Is it a JIT, a GPU kernel,
/// or is it optimizing for low instructions...
/// or fast many instructions.
#[derive(Copy,Clone,Debug)]
pub enum CodeModel {
  Default,
  JIT,
  Small,
  Medium,
  Large,
  Kernel
}
impl Into<LLVMCodeModel> for CodeModel {
  fn into(self) -> LLVMCodeModel {
    match self {
      CodeModel::Default => LLVMCodeModel::LLVMCodeModelDefault,
      CodeModel::JIT => LLVMCodeModel::LLVMCodeModelJITDefault,
      CodeModel::Small => LLVMCodeModel::LLVMCodeModelSmall,
      CodeModel::Medium => LLVMCodeModel::LLVMCodeModelMedium,
      CodeModel::Large => LLVMCodeModel::LLVMCodeModelLarge,
      CodeModel::Kernel => LLVMCodeModel::LLVMCodeModelKernel,
    }
  }
}
impl Default for CodeModel {
  fn default() -> Self {
    CodeModel::Default
  }
}

/// Target Machine
/// 
/// Describe the physical machine that is being
/// compiled too. 
pub struct TargetMachine {
  data: LLVMTargetMachineRef,
  buffers: Vec<Buffers>
}
impl Drop for TargetMachine {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeTargetMachine(self.data);
    }
  }
}
impl Default for TargetMachine {
  
  /// Create a target machine using purely default values.
  ///
  /// This will use all the default code relocation, code
  /// model, and code relocation directives.
  ///
  /// It'll assume the CPU has NO special features
  /// 
  /// It'll assume the CPU is completely generic
  fn default() -> Self {
    let cpu = CString::new("generic").unwrap();
    let features = CString::new("").unwrap();
    let t_ptr = Target::default();
    let (tar,buffs) = unsafe{t_ptr.split()};
    let triple = get_local_triple();
    let tm_ptr = unsafe{
      LLVMCreateTargetMachine(
        tar,
        triple.as_ptr() as *const _,
        cpu.as_ptr() as *const _,
        features.as_ptr() as *const _,
        CodeGenOptLevel::default().into(),
        RelocMode::default().into(),
        CodeModel::default().into()
      )
    };
    let mut buffers = Vec::with_capacity(10);
    buffers.push(Buffers::A(cpu));
    buffers.push(Buffers::A(features));
    buffers.push(Buffers::A(triple));
    buffers.extend(buffs);
    TargetMachine {
      data: tm_ptr,
      buffers: buffers
    } 
  }
}
impl TargetMachine {
  
  /// Method internal to the library.
  pub unsafe fn from_raw(data: LLVMTargetMachineRef, buff: Vec<Buffers>) -> Self {
    TargetMachine {
      data: data,
      buffers: buff
    }
  }

  pub fn get_cpu(&self) -> CString {
    unsafe{
      let a = LLVMGetTargetMachineCPU(self.data);
      let ptr = CStr::from_ptr(a).to_owned();
      LLVMDisposeMessage(a);
      ptr
    }
  }
  pub fn get_features(&self) -> CString {
    unsafe {
      let a = LLVMGetTargetMachineFeatureString(self.data);
      let ptr = CStr::from_ptr(a).to_owned();
      LLVMDisposeMessage(a);
      ptr
    }
  }
  pub fn get_triple(&self) -> CString {
    unsafe {
      let a = LLVMGetTargetMachineTriple(self.data);
      let ptr = CStr::from_ptr(a).to_owned();
      LLVMDisposeMessage(a);
      ptr
    }
  }

  /// I don't know what this does
  pub fn set_asm_verbose(&mut self, flag: bool) {
    let llvmbool = if flag { 1 } else { 0 };
    unsafe{
      LLVMSetTargetMachineAsmVerbosity(self.data,llvmbool);
    }
  } 
 
}

/// Target Machine Builder
///
/// This allows for using the builder pattern to build
/// TargetMachine. Effecively the function signature
/// is massive so this makes construction more
/// understandable.
pub struct BuildTargetMachine {
  cpu: CString,
  features: CString,
  target: Target,
  triple: CString,
  gen_opt: CodeGenOptLevel,
  reloc_mode: RelocMode,
  code_model: CodeModel,
}
impl BuildTargetMachine {
  
  /// This sets the default options.
  ///
  /// On it's own calling new here and
  /// immidately building is no different
  /// then just calling `TargetMachine::default()`
  pub fn new() -> Self {
    BuildTargetMachine {
      cpu: CString::new("generic").unwrap(),
      features: CString::new("").unwrap(),
      target: Target::default(),
      triple: get_local_triple(),
      gen_opt: CodeGenOptLevel::default(),
      reloc_mode: RelocMode::default(),
      code_model: CodeModel::default()
    }
  }

  pub fn set_features(&mut self, features: &str) -> &mut Self {
    let cstr = CString::new(features).expect(NULLPTR);
    self.features = cstr;
    self
  }

  pub fn set_cpu(&mut self, cpu: &str) -> &mut Self {
    let cstr = CString::new(cpu).expect(NULLPTR);
    self.cpu = cstr;
    self
  }

  pub fn set_target_triple(&mut self, triple: &str) -> &mut Self {
    let cstr = CString::new(triple).expect(NULLPTR);
    self.triple = cstr;
    self
  }

  pub fn set_opt_level(&mut self, opt: CodeGenOptLevel) -> &mut Self {
    self.gen_opt = opt;
    self
  }

  pub fn set_reloc_mode(&mut self, reloc: RelocMode) -> &mut Self {
    self.reloc_mode = reloc;
    self
  }

  pub fn set_code_model(&mut self, model: CodeModel) -> &mut Self {
    self.code_model = model;
    self
  }
  pub fn set_target(&mut self, target: Target) -> &mut Self {
    self.target = target;
    self
  }
  pub fn build(self) -> TargetMachine {
    let mut buffers = Vec::with_capacity(10);
    unsafe{
      
      let (tar,buffs) = self.target.split();
      let tm_ptr = LLVMCreateTargetMachine(
        tar,
        self.triple.as_ptr() as *const _,
        self.cpu.as_ptr() as *const _,
        self.features.as_ptr() as *const _,
        self.gen_opt.into(),
        self.reloc_mode.into(),
        self.code_model.into()
      );
      buffers.extend(buffs);
      buffers.push(Buffers::A(self.cpu));
      buffers.push(Buffers::A(self.features));
      buffers.push(Buffers::A(self.triple));
      TargetMachine::from_raw(tm_ptr, buffers)
    }
  }
}
    


































































 
