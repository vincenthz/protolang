use werbolg_compile::compile;
use werbolg_core::Module;
use werbolg_exec::{ExecutionEnviron, ExecutionParams};
use werbolg_lang_rusty::module;

use std::{fs, path::Path};

mod value;

pub use value::Value;

#[derive(Clone)]
pub struct ProtocolState {}

pub struct DummyAlloc;

pub type ExecutionMachine<'m, 'e> =
    werbolg_exec::ExecutionMachine<'m, 'e, DummyAlloc, (), ProtocolState, Value>;

pub struct Actor {}

pub fn sources<S: AsRef<Path>>(dir: S) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let mut modules = Vec::new();
    for file in fs::read_dir(dir).unwrap() {
        let dir_ent = file.unwrap();
        // todo
    }
    Ok(modules)
}
