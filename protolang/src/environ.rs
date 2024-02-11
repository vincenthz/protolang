use crate::{Allocator, Environment, ExecutionMachine, ProtocolState, Value};
use num_traits::identities::Zero;
use werbolg_compile::CallArity;
use werbolg_core::{AbsPath, Ident, Literal, Namespace};
use werbolg_exec::{ExecutionError, NIFCall};

fn nil(em: &mut ExecutionMachine) -> Result<Value, ExecutionError> {
    Ok(Value::Unit)
}

fn broadcast(em: &mut ExecutionMachine) -> Result<Value, ExecutionError> {
    let (_, args) = em.stack.get_call_and_args(em.current_arity);
    let mut args = args.iter();
    let message = args.next().unwrap();
    let value = args.next().unwrap();

    println!("broadcasting message: {message:?}");
    // broadcast the message: em.userdata;

    Ok(Value::Unit)
}

fn num_plus(_alloc: &Allocator, args: &[Value]) -> Result<Value, ExecutionError> {
    let a0 = args[0].int()?;
    let a1 = args[1].int()?;
    let res = Value::Integral(a0 + a1);
    Ok(res)
}

fn num_minus(_alloc: &Allocator, args: &[Value]) -> Result<Value, ExecutionError> {
    let a0 = args[0].int()?;
    let a1 = args[1].int()?;
    let res = Value::Integral(a0 - a1);
    Ok(res)
}

fn num_mul(_alloc: &Allocator, args: &[Value]) -> Result<Value, ExecutionError> {
    let a0 = args[0].int()?;
    let a1 = args[1].int()?;
    let res = Value::Integral(a0 * a1);
    Ok(res)
}

fn num_div(_alloc: &Allocator, args: &[Value]) -> Result<Value, ExecutionError> {
    let a0 = args[0].int()?;
    let a1 = args[1].int()?;
    if a1.is_zero() {
        return Err(ExecutionError::UserPanic {
            message: format!("trying to divide by 0"),
        });
    }
    let res = Value::Integral(a0 / a1);
    Ok(res)
}

fn num_mod(_alloc: &Allocator, args: &[Value]) -> Result<Value, ExecutionError> {
    let a0 = args[0].int()?;
    let a1 = args[1].int()?;
    if a1.is_zero() {
        return Err(ExecutionError::UserPanic {
            message: format!("trying to modulo by 0"),
        });
    }
    let res = Value::Integral(a0 % a1);
    Ok(res)
}

pub fn module_arithmetics() -> Vec<NIFCall<Allocator, Literal, ProtocolState, Value>> {
    vec![
        NIFCall::Pure(num_plus),
        NIFCall::Pure(num_minus),
        NIFCall::Pure(num_mul),
        NIFCall::Pure(num_div),
        NIFCall::Pure(num_mod),
    ]
}

pub fn create_env() -> Environment {
    macro_rules! add_raw_nif {
        ($env:ident, $i:literal, $arity:literal, $e:expr) => {
            let nif = NIFCall::Raw($e).info($i, CallArity::try_from($arity as usize).unwrap());
            let path = AbsPath::new(&Namespace::root(), &Ident::from($i));
            $env.add_nif(&path, nif);
        };
    }

    let mut env = Environment::new();
    add_raw_nif!(env, "nil", 0, nil);
    add_raw_nif!(env, "broadcast", 2, broadcast);

    env
}
