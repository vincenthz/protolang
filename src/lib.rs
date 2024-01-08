use werbolg_compile::compile;
use werbolg_exec::{ExecutionEnviron, ExecutionParams};
use werbolg_lang_rusty::module;

mod value;

pub use value::Value;

#[derive(Clone)]
pub struct ProtocolState {}

pub struct DummyAlloc;

pub type ExecutionMachine<'m, 'e> =
    werbolg_exec::ExecutionMachine<'m, 'e, DummyAlloc, (), ProtocolState, Value>;

pub struct Actor {}