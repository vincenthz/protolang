mod allocator;
mod environ;
mod value;

use std::{error::Error, fs, path::Path};
use werbolg_core::{Literal, Module, Namespace};
use werbolg_lang_common::FileUnit;
use werbolg_lang_rusty as rusty;

pub use self::{allocator::Allocator, environ::create_env, value::Value};

pub type NIF<'m, 'e> = werbolg_exec::NIF<'m, 'e, Allocator, Literal, ProtocolState, Value>;
pub type Environment<'m, 'e> = werbolg_compile::Environment<NIF<'m, 'e>, Value>;
pub type ExecutionMachine<'m, 'e> =
    werbolg_exec::ExecutionMachine<'m, 'e, Allocator, Literal, ProtocolState, Value>;

#[derive(Clone)]
pub struct ProtocolState {}

pub struct Actor {}

pub fn sources<S: AsRef<Path>>(dir: S) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let mut modules = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        let entry_type = entry.file_type().unwrap();
        if entry_type.is_file() {
            let unit = get_file(entry.path()).unwrap();
            let module = rusty::module(&unit).expect("Parse file");
            modules.push(module);
        } else if entry_type.is_dir() {
            let path = entry.path();
            panic!("Directory not supported yet: {}", path.display())
        }
    }
    Ok(modules)
}

pub fn compiles(
    env: &mut Environment<'_, '_>,
    modules: Vec<Module>,
) -> Result<Vec<werbolg_compile::CompilationUnit<Literal>>, Box<dyn Error>> {
    let mut compiled = Vec::new();

    for module in modules {
        compiled.push(compile(env, module)?);
    }

    Ok(compiled)
}

pub fn compile(
    env: &mut Environment<'_, '_>,
    module: Module,
) -> Result<werbolg_compile::CompilationUnit<Literal>, Box<dyn Error>> {
    let module_ns = Namespace::root();
    let modules = vec![(module_ns.clone(), module)];

    let compilation_params = werbolg_compile::CompilationParams { literal_mapper: Ok };

    let exec_module = werbolg_compile::compile(&compilation_params, modules, env)
        .map_err(|e| format!("compilation error {:?}", e))?;

    Ok(exec_module)
}

fn get_file<P: AsRef<Path>>(path: P) -> std::io::Result<FileUnit> {
    let path = path.as_ref();
    let filename = path.file_name().expect("File Name");

    let content = std::fs::read_to_string(path).expect("file read");
    let fileunit = FileUnit::from_string(filename.to_string_lossy().to_string(), content);
    Ok(fileunit)
}
