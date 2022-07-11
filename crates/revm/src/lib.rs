#![allow(dead_code)]
//#![no_std]

pub mod db;
mod evm;
mod evm_impl;
pub(crate) mod gas;
mod inspector;
mod instructions;
mod interpreter;
mod models;
mod specification;
mod subroutine;

pub use evm_impl::{EVMData, Host};

pub type DummyStateDB = InMemoryDB;

pub use db::{Database, DatabaseCommit, InMemoryDB};
pub use evm::{evm_inner, new, EVM};
pub use gas::Gas;
pub use inspector::{Inspector, NoOpInspector, OverrideSpec};
pub use instructions::{
    opcode::{self, spec_opcode_gas, OpCode, OPCODE_JUMPMAP},
    Return,
};
pub use interpreter::{Contract, Interpreter, Memory, Stack};
pub use models::*;
pub use specification::*;
pub use subroutine::{Account, Filth, SubRoutine};

extern crate alloc;

pub(crate) const USE_GAS: bool = !cfg!(feature = "no_gas_measuring");


#[repr(C)]
#[derive(Debug)]
pub struct RetUint {
    pub n1: u64,
    pub n2: u64,
    pub n3: u64,
    pub n4: u64,
}

#[link(name = "intx")]
extern "C" {
    pub fn fast_div_rem(f: *const u64, s: *const u64) -> RetUint;
}


use primitive_types::{U256,H256};

pub fn test_it() {
    let f = U256::from_big_endian(H256::from_low_u64_be(20).as_ref());
    let s = U256::from_big_endian(H256::from_low_u64_be(10).as_ref());

    let t = unsafe { fast_div_rem(f.as_ref().as_ptr(),s.as_ref().as_ptr())};
    println!("TEST_IT:{:?}",t);

} 
