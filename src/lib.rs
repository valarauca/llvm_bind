//! LLVM Bind
//!
//! This crate creates the semantics of the LLVM C++ in Rust.
//!
//! It provides a safe idomatic Rust layer infront of
//! llvm_sys library.
//!
//! This library was written and tested on x86_64 linux
//! with llvm_sys = 0.4.0 If you try to get to work
//! with different versions things may get a bit
//! fuzzy.


extern crate llvm_sys;
use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::analysis::*;
use llvm_sys::bit_writer::*;
use llvm_sys::target_machine::*;

use std::ffi::{CString,CStr};
use std::os::raw::c_char;



/// LLVM Named Buffers
///
/// For holding IR and Strings
pub mod buffer;

/// LLVM Module
///
/// Modules are units of compilation.
/// They are interacted with to build up
/// LLVM-IR.
///
/// Buffers can be parsed into Modules.
///
/// Modules can be converted into text and written to a
/// buffer.
pub mod module;


/// LLVM Target information
///
/// Contains information relating to the target of compilation
pub mod target;

/// LLVM Pass information
///
/// Contains what optimization passes should be performed
/// on the to be compiled module.
pub mod pass;

/// Used in internal data structures to keep things alive
///
/// llvm_sys, nor the LLVM's C API, nor the LLVM itself are
/// to particular on memory management. So this enum is used
/// to keep objects alive which are tied to an LLVM pointer.
pub enum Buffers {
    A(CString),
    B(Vec<u8>),
}









