mod allocator;
mod environ;
mod value;

use std::{error::Error, fs, path::Path};
use werbolg_core::{Ident, Literal, Module, Namespace};
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

pub fn source<S: AsRef<Path>>(path: S) -> Result<Module, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path.as_ref()).expect("file read");
    let file_unit = FileUnit::from_string(path.as_ref().display().to_string(), content);

    let module = rusty::module(&file_unit).map_err(|e| format!("{:?}", e))?;
    Ok(module)
}

/// Load all the modules in the given path
pub fn sources<S: AsRef<Path>>(
    dir: S,
) -> Result<Vec<(Namespace, Module)>, Box<dyn std::error::Error>> {
    let mut modules = Vec::new();
    let base = dir.as_ref().to_path_buf();

    let dir_namespace = Namespace::root();

    for file in fs::read_dir(dir)? {
        let dir_ent = file?;
        let dir_ent_type = dir_ent.file_type()?;
        if dir_ent_type.is_file() {
            let mut base = base.clone();
            base.push(dir_ent.file_name());
            let filename = dir_ent.file_name();
            let s = filename.to_str().ok_or("")?;
            if let Some(module_name) = s.strip_suffix(".protolang") {
                let i = Ident::from(module_name);
                let module_namespace = dir_namespace.clone().append(i);
                let m = source(base)?;
                modules.push((module_namespace, m))
            }
        } else if dir_ent_type.is_dir() {
            // todo recurse
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
