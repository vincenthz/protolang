mod allocator;
mod environ;
mod value;

use std::{fs, path::Path, process::exit};
use werbolg_compile::CompilationState;
use werbolg_core::{Ident, Literal, Module, Namespace};
use werbolg_lang_common::Source;
use werbolg_lang_rusty as rusty;

pub use self::{allocator::Allocator, environ::create_env, value::Value};

pub type NIF = werbolg_exec::NIF<Allocator, Literal, ProtocolState, Value>;
pub type Environment = werbolg_compile::Environment<NIF, Value>;
pub type ExecutionMachine =
    werbolg_exec::ExecutionMachine<Allocator, Literal, ProtocolState, Value>;

use werbolg_lang_common::{Report, ReportKind};

#[derive(Clone)]
pub struct ProtocolState {}

pub struct Actor {}

fn source_api<S: AsRef<Path>>(path: S) -> std::io::Result<Source> {
    let content = std::fs::read_to_string(path.as_ref()).expect("file read");
    let source = Source::from_string(path.as_ref().display().to_string(), content);
    Ok(source)
}

/// Load all the modules in the given path
fn sources_api<S: AsRef<Path>>(dir: S) -> std::io::Result<Vec<(Namespace, Source)>> {
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
            let s = filename.to_str().ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "filename is not unicode",
            ))?;
            if let Some(module_name) = s.strip_suffix(".protolang") {
                let i = Ident::from(module_name);
                let module_namespace = dir_namespace.clone().append(i);
                let m = source_api(base)?;
                modules.push((module_namespace, m))
            }
        } else if dir_ent_type.is_dir() {
            // todo recurse
        }
    }
    Ok(modules)
}

pub fn sources<S: AsRef<Path>>(dir: S) -> Vec<(Namespace, Source, Module)> {
    let path_display = format!("{}", dir.as_ref().display());
    let sources = match sources_api(dir) {
        Err(r) => {
            eprintln!("sources I/O error in {}\n{:?}", r, path_display);
            exit(1)
        }
        Ok(t) => t,
    };

    sources
        .into_iter()
        .map(|(n, source)| match rusty::module(&source.file_unit) {
            Err(perrs) => {
                for e in perrs.into_iter() {
                    let report = Report::new(ReportKind::Error, format!("Parse Error: {:?}", e))
                        .lines_before(1)
                        .lines_after(1)
                        .highlight(e.location, format!("parse error here"));

                    let mut s = String::new();
                    report
                        .write(&source, &mut s)
                        .expect("write to string works");
                    eprintln!("{}", s);
                }
                /*
                let report = Report::new(ReportKind::Error, format!("Parse Error: {:?}", perr))
                    .lines_before(1)
                    .lines_after(1)
                    .highlight(perr.location, format!("parse error here"));

                let mut s = String::new();
                report
                    .write(&source, &mut s)
                    .expect("write to string works");
                eprintln!("{}", s);
                */
                exit(1)
            }
            Ok(module) => (n, source, module),
        })
        .collect()
}

pub fn compile(
    env: &mut Environment,
    modules: Vec<(Namespace, Source, Module)>,
) -> werbolg_compile::CompilationUnit<Literal> {
    let compilation_params = werbolg_compile::CompilationParams {
        literal_mapper: |_, x| Ok(x),
        sequence_constructor: None,
    };

    let mut compiler = CompilationState::new(compilation_params);
    for (ns, _, module) in modules.into_iter() {
        match compiler.add_module(&ns, module) {
            Err(e) => {
                eprintln!("compilation error while adding module {:?}\n{:?}", ns, e);
                exit(1)
            }
            Ok(()) => {}
        }
    }

    match compiler.finalize(env) {
        Err(e) => {
            eprintln!("compilation error while generating code\n{:?}", e);
            exit(1)
        }
        Ok(cunit) => cunit,
    }
}
