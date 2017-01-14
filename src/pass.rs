

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::transforms::vectorize::*;
use llvm_sys::transforms::scalar::*;
use llvm_sys::transforms::ipo::*;


macro_rules! pass {
  ($name: ident, $pass: ident) => {
    pub fn $name(&mut self) {
      unsafe{
          $pass(self.data);
      }
    }
  }
}

/// Manages code gen passes
pub struct PassManager {
  data: LLVMPassManagerRef
}
impl Drop for PassManager {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposePassManager(self.data);
    }
  }
}
impl PassManager {

  /// Creates a new empty Pass Manager
  pub fn new() -> Self {
    unsafe{
      PassManager {
        data: LLVMCreatePassManager()
      }
    }
  }
  

  /// Gives the option to the end developer to
  /// handle some optimizations more eloquently
  pub fn apply_opt<T: ApplyOpt>(&mut self, opt: T) {
      opt.add_pass(self);
  }

  pass!(aggressive_dce,LLVMAddAggressiveDCEPass);
  pass!(alignment_from_assumptions,LLVMAddAlignmentFromAssumptionsPass);
  pass!(cfg_simplification,LLVMAddCFGSimplificationPass);
  pass!(constant_propigation,LLVMAddConstantPropagationPass);
  pass!(correlated_value_propagation,LLVMAddCorrelatedValuePropagationPass);
  pass!(dead_store_elimination,LLVMAddDeadStoreEliminationPass);
  pass!(demote_memory_to_register,LLVMAddDemoteMemoryToRegisterPass);
  pass!(early_cse,LLVMAddEarlyCSEPass);
  pass!(gvn,LLVMAddGVNPass);
  pass!(ind_var_simplify,LLVMAddIndVarSimplifyPass);
  pass!(instruction_combining,LLVMAddInstructionCombiningPass);
  pass!(jump_threading,LLVMAddJumpThreadingPass);
  pass!(licm,LLVMAddLICMPass);
  pass!(loop_deletion,LLVMAddLoopDeletionPass);
  pass!(loop_reroll,LLVMAddLoopRerollPass);
  pass!(loop_rotate,LLVMAddLoopRotatePass);
  pass!(loop_unswitch,LLVMAddLoopUnswitchPass);
  pass!(lower_expect_intrinsic,LLVMAddLowerExpectIntrinsicPass);
  pass!(lower_switch,LLVMAddLowerSwitchPass);
  pass!(memcpy_opt,LLVMAddMemCpyOptPass);
  pass!(merge_load_store_motion,LLVMAddMergedLoadStoreMotionPass);
  pass!(partially_inline_lib_calls,LLVMAddPartiallyInlineLibCallsPass);
  pass!(promote_memory_to_register,LLVMAddPromoteMemoryToRegisterPass);
  pass!(reassociate,LLVMAddReassociatePass);
  pass!(sccp,LLVMAddSCCPPass);
  pass!(scalarizer,LLVMAddScalarizerPass);
  pass!(scoped_no_alias_aa,LLVMAddScopedNoAliasAAPass);
  pass!(simplify_lib_calls,LLVMAddSimplifyLibCallsPass);
  pass!(tail_call_elimination,LLVMAddTailCallEliminationPass);
  pass!(verifer,LLVMAddVerifierPass);
  pass!(always_inliner,LLVMAddAlwaysInlinerPass);
  pass!(argument_promotion,LLVMAddArgumentPromotionPass);
  pass!(constant_merge,LLVMAddConstantMergePass);
  pass!(dead_arg_elimination,LLVMAddDeadArgEliminationPass);
  pass!(function_attrs,LLVMAddFunctionAttrsPass);
  pass!(global_dce,LLVMAddGlobalDCEPass);
  pass!(global_optimizer,LLVMAddGlobalOptimizerPass);
  pass!(ip_constant_propagation,LLVMAddIPConstantPropagationPass);
  pass!(ipsccp,LLVMAddIPSCCPPass);
  pass!(prune,LLVMAddPruneEHPass);
  pass!(strip_dead_prototypes,LLVMAddStripDeadPrototypesPass);
  pass!(strip_symbols,LLVMAddStripSymbolsPass);
  
  pub fn internalize_pass(&mut self, all_but_main: u32) {
    unsafe{
      LLVMAddInternalizePass(self.data, all_but_main);
    }
  }

  /// Allows access to inner data field
  /// for within library functions
  pub unsafe fn inner(&mut self) -> LLVMPassManagerRef {
    self.data
  }
}


/// For enums that can apply optimizations 
pub trait ApplyOpt {
  fn add_pass(&self, mngr: &mut PassManager);
}

/// Controls which `ScalarReplAggregatesPass`
/// is performed
///
/// you can re-call the method to apply more
/// then one type.
#[derive(Clone,Copy,Debug)]
pub enum ScalarReplAggregates {
  /// applies no optimization pass
  None,
  /// equalivant of `LLVMAddScalarReplAggregatesPass`
  Default,
  /// equalivant of `LLVMAddScalarReplAggregatesPassSSA`
  SSA,
  /// equalivant of `LLVMAddScalarReplAggregatesWithThreshold`
  Threshold(i32)
}
impl ApplyOpt for ScalarReplAggregates {
  fn add_pass(&self, mngr: &mut PassManager) {
    unsafe { 
      match *self {
        ScalarReplAggregates::None => { },
        ScalarReplAggregates::Default => LLVMAddScalarReplAggregatesPass(mngr.inner()),
        ScalarReplAggregates::SSA => LLVMAddScalarReplAggregatesPassSSA(mngr.inner()),
        ScalarReplAggregates::Threshold(val) => LLVMAddScalarReplAggregatesPassWithThreshold(mngr.inner(),val)
      };
    }
  }
}

/// Vectorization Passes
///
/// you can add multiple
/// by re-calling the `appy_opt`
/// value multiple times.
#[derive(Clone,Copy,Debug)]
pub enum Vectorize {
  /// applies no optimization pass
  None,
  /// equalivant of `LLVMAddBBVectorizePass`
  BB,
  /// equalivant of `LLVMAddLoopVectorizePass`
  Loop,
  /// equalivant of `LLVMAddSLPVectorizePass`
  SLP
}
impl ApplyOpt for Vectorize {
  fn add_pass(&self, mngr: &mut PassManager) {
    unsafe { 
      match *self {
        Vectorize::None => { },
        Vectorize::BB => LLVMAddBBVectorizePass(mngr.inner()),
        Vectorize::Loop => LLVMAddLoopVectorizePass(mngr.inner()),
        Vectorize::SLP => LLVMAddSLPVectorizePass(mngr.inner())
      };
    }
  }
}



























































