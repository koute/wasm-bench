#[cfg(feature = "wasmi_016")]
fn run_under_wasmi<F>(bytecode: &[u8], entry_point: &str, clock_ms: F) -> f32
    where F: Fn() -> i64 + Send + Sync + 'static
{
    struct HostState;

    use wasmi_016::*;

    let engine = Engine::default();
    let module = Module::new(&engine, bytecode).unwrap();
    let mut store = Store::new(&engine, HostState);
    let mut linker = Linker::<HostState>::new();

    let clock_ms = Func::wrap(&mut store, move |_caller: Caller<'_, HostState>| -> i64 { clock_ms() });
    linker.define("env", "clock_ms", clock_ms).unwrap();

    let instance = linker
        .instantiate(&mut store, &module).unwrap()
        .start(&mut store).unwrap();

    let run = instance
        .get_export(&store, entry_point)
        .and_then(Extern::into_func)
        .unwrap()
        .typed::<(), core::F32, _>(&mut store)
        .unwrap();

    run.call(&mut store, ()).unwrap().to_float()
}

#[cfg(
    any(
        feature = "wasmi_009",
        feature = "wasmi_013"
    )
)]
fn run_under_wasmi<F>(bytecode: &[u8], entry_point: &str, clock_ms: F) -> f32
    where F: Fn() -> i64 + Send + Sync + 'static
{
    #[cfg(feature = "wasmi_013")]
    use wasmi_013::*;

    #[cfg(feature = "wasmi_009")]
    use wasmi_009::*;

    struct EnvModuleResolver(FuncRef);
    impl ModuleImportResolver for EnvModuleResolver {
        fn resolve_func(
            &self,
            field_name: &str,
            _signature: &Signature
        ) -> Result<FuncRef, Error> {
            if field_name == "clock_ms" {
                Ok(self.0.clone())
            } else {
                Err(Error::Instantiation(format!(
                    "Export {} not found",
                    field_name
                )))
            }
        }
    }

    struct ExternalsStruct<F>(F) where F: Fn() -> i64 + Send + Sync + 'static;
    impl<F> Externals for ExternalsStruct<F> where F: Fn() -> i64 + Send + Sync + 'static {
        fn invoke_index(
            &mut self,
            index: usize,
            _args: RuntimeArgs<'_>
        ) -> Result<Option<RuntimeValue>, Trap> {
            if index == 0 {
                Ok(Some(RuntimeValue::I64((self.0)())))
            } else {
                #[cfg(feature = "wasmi_009")]
                { Err(Trap::new(TrapKind::Unreachable)) }

                #[cfg(feature = "wasmi_013")]
                { Err(Trap::Code(TrapCode::Unreachable)) }
            }
        }
    }

    let signature = Signature::new(&[][..], Some(ValueType::I64));
    let func_clock_ms = FuncInstance::alloc_host(signature, 0);
    let resolver = EnvModuleResolver(func_clock_ms);

    let module = Module::from_buffer(bytecode).unwrap();
    let imports = ImportsBuilder::new().with_resolver("env", &resolver);
    let instance = ModuleInstance::new(&module, &imports).unwrap().assert_no_start();

    let mut externals = ExternalsStruct(clock_ms);

    let value = instance.invoke_export(
        entry_point,
        &[],
        &mut externals,
    ).unwrap();

    match value {
        Some(RuntimeValue::F32(value)) => value.to_float(),
        _ => unreachable!()
    }
}

pub fn run_coremark_impl<F>(clock_ms: F) -> f32
    where F: Fn() -> i64 + Send + Sync + 'static
{
    // Source: https://github.com/patractlabs/wasm-coremark-rs
    let bytecode = include_bytes!("coremark-minimal.wasm");
    run_under_wasmi(bytecode, "run", clock_ms)
}

pub fn run_regexredux_impl<F>(clock_ms: F) -> f32
    where F: Fn() -> i64 + Send + Sync + 'static
{
    let bytecode = include_bytes!("../../target/wasm32-unknown-unknown/lto/kernel_regexredux.wasm");
    run_under_wasmi(bytecode, "run_regexredux", clock_ms)
}

pub fn run_noop_impl<F>(clock_ms: F) -> f32
    where F: Fn() -> i64 + Send + Sync + 'static + Clone
{
    let bytecode = include_bytes!("../../target/wasm32-unknown-unknown/lto/kernel_noop.wasm");
    let timestamp = clock_ms();
    for _ in 0..10000 {
        run_under_wasmi(bytecode, "run_noop", clock_ms.clone());
    }

    let elapsed = clock_ms() - timestamp;
    elapsed as f32
}

#[cfg(target_arch = "wasm32")]
extern {
    fn clock_ms() -> i64;
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn run_coremark() -> f32 {
    run_coremark_impl(|| unsafe { clock_ms() })
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn run_regexredux() -> f32 {
    run_regexredux_impl(|| unsafe { clock_ms() })
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn run_noop() -> f32 {
    run_noop_impl(|| unsafe { clock_ms() })
}
