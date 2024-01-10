use crate::{Environment, ExecutionMachine, Value, NIF};
use werbolg_core::{AbsPath, Ident, Namespace};
use werbolg_exec::{ExecutionError, NIFCall};

fn broadcast(em: &mut ExecutionMachine<'_, '_>) -> Result<Value, ExecutionError> {
    let (_, args) = em.stack.get_call_and_args(em.current_arity);
    let mut args = args.iter();
    let Some(message) = args.next() else {
        return Err(ExecutionError::UserPanic {
            message:
                "Expecting a message to be broadcasted, none passed to the `broadcast' function."
                    .to_string(),
        });
    };
    if let Some(value) = args.next() {
        return Err(ExecutionError::UserPanic {
            message: format!("Unexpected parameter to broadcast function: {value:?}"),
        });
    }

    println!("broadcasting message: {message:?}");
    // broadcast the message: em.userdata;

    Ok(Value::Unit)
}

pub fn create_env<'m, 'e>() -> Environment<'m, 'e> {
    macro_rules! add_raw_nif {
        ($env:ident, $i:literal, $e:expr) => {
            let nif = NIF {
                name: $i,
                call: NIFCall::Raw($e),
            };
            let path = AbsPath::new(&Namespace::root(), &Ident::from($i));
            $env.add_nif(&path, nif);
        };
    }

    let mut env = Environment::new();
    add_raw_nif!(env, "broadcast", broadcast);

    env
}
