fn clock_ms() -> impl Fn() -> i64 + Clone {
    let start = std::time::Instant::now();
    move || {
        start.elapsed().as_millis() as i64
    }
}

#[cfg(any(feature = "target_wasmi_under_wasmtime", feature = "target_wasmtime"))]
const WASMTIME_VERSION: &'static str = {
    #[cfg(feature = "wasmtime_040")] { "wasmtime_040" }
    #[cfg(feature = "wasmtime_039")] { "wasmtime_039" }
    #[cfg(feature = "wasmtime_038")] { "wasmtime_038" }
    #[cfg(feature = "wasmtime_037")] { "wasmtime_037" }
    #[cfg(feature = "wasmtime_036")] { "wasmtime_036" }
    #[cfg(feature = "wasmtime_035")] { "wasmtime_035" }
    #[cfg(feature = "wasmtime_034")] { "wasmtime_034" }
    #[cfg(feature = "wasmtime_033")] { "wasmtime_033" }
};

#[cfg(any(feature = "target_wasmi_on_bare_metal", feature = "target_wasmi_under_wasmtime"))]
const WASMI_VERSION: &'static str = {
    #[cfg(feature = "wasmi_016")] { "wasmi_016" }
    #[cfg(feature = "wasmi_013")] { "wasmi_013" }
    #[cfg(feature = "wasmi_009")] { "wasmi_009" }
};

fn target() -> String {
    #[cfg(feature = "target_wasmi_under_wasmtime")]
    { format!("{}_under_{}", WASMI_VERSION, WASMTIME_VERSION) }

    #[cfg(feature = "target_wasmi_on_bare_metal")]
    { format!("{}_on_bare_metal", WASMI_VERSION) }

    #[cfg(feature = "target_wasmtime")]
    { WASMTIME_VERSION.into() }

    #[cfg(feature = "target_bare_metal")]
    { "bare_metal".into() }
}

#[cfg(any(feature = "target_wasmi_under_wasmtime", feature = "target_wasmtime"))]
#[cfg(any(
    feature = "wasmtime_040",
    feature = "wasmtime_039",
    feature = "wasmtime_038",
    feature = "wasmtime_037",
    feature = "wasmtime_036",
    feature = "wasmtime_035",
    feature = "wasmtime_034",
    feature = "wasmtime_033",
))]
fn run_on_wasmtime(bytecode: &[u8], entry_point: &str) -> f32 {
    #[cfg(feature = "wasmtime_040")] use wasmtime_040::*;
    #[cfg(feature = "wasmtime_039")] use wasmtime_039::*;
    #[cfg(feature = "wasmtime_038")] use wasmtime_038::*;
    #[cfg(feature = "wasmtime_037")] use wasmtime_037::*;
    #[cfg(feature = "wasmtime_036")] use wasmtime_036::*;
    #[cfg(feature = "wasmtime_035")] use wasmtime_035::*;
    #[cfg(feature = "wasmtime_034")] use wasmtime_034::*;
    #[cfg(feature = "wasmtime_033")] use wasmtime_033::*;

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let clock_ms = clock_ms();
    linker.func_wrap("env", "clock_ms", move |_caller: Caller<'_, ()>| -> i64 { clock_ms() }).unwrap();

    let module = Module::new(&engine, bytecode).unwrap();
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let run = instance.get_typed_func::<(), f32, _>(&mut store, entry_point).unwrap();
    run.call(&mut store, ()).unwrap()
}

#[cfg(feature = "target_wasmi_under_wasmtime")]
#[cfg(feature = "wasmi_016")]
static KERNEL_WASMI: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/lto-wasmi_016/kernel_wasmi.wasm");

#[cfg(feature = "target_wasmi_under_wasmtime")]
#[cfg(feature = "wasmi_013")]
static KERNEL_WASMI: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/lto-wasmi_013/kernel_wasmi.wasm");

#[cfg(feature = "target_wasmi_under_wasmtime")]
#[cfg(feature = "wasmi_009")]
static KERNEL_WASMI: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/lto-wasmi_009/kernel_wasmi.wasm");

#[cfg(feature = "target_wasmtime")]
#[cfg(feature = "bench_coremark")]
static KERNEL_COREMARK: &[u8] = include_bytes!("../kernel_wasmi/src/coremark-minimal.wasm");

#[cfg(feature = "target_wasmtime")]
#[cfg(feature = "bench_regexredux")]
static KERNEL_REGEXREDUX: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/lto/kernel_regexredux.wasm");

#[cfg(feature = "target_wasmtime")]
#[cfg(feature = "bench_noop")]
static KERNEL_NOOP: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/lto/kernel_noop.wasm");

// target_wasmi_under_wasmtime

#[cfg(feature = "bench_coremark")]
#[cfg(feature = "target_wasmi_under_wasmtime")]
fn run_coremark() -> f32 {
    run_on_wasmtime(KERNEL_WASMI, "run_coremark")
}

#[cfg(feature = "bench_regexredux")]
#[cfg(feature = "target_wasmi_under_wasmtime")]
fn run_regexredux() -> f32 {
    run_on_wasmtime(KERNEL_WASMI, "run_regexredux")
}

#[cfg(feature = "bench_noop")]
#[cfg(feature = "target_wasmi_under_wasmtime")]
fn run_noop() -> f32 {
    run_on_wasmtime(KERNEL_WASMI, "run_noop")
}

// target_wasmtime

#[cfg(feature = "bench_coremark")]
#[cfg(feature = "target_wasmtime")]
fn run_coremark() -> f32 {
    run_on_wasmtime(KERNEL_COREMARK, "run")
}

#[cfg(feature = "bench_regexredux")]
#[cfg(feature = "target_wasmtime")]
fn run_regexredux() -> f32 {
    run_on_wasmtime(KERNEL_REGEXREDUX, "run_regexredux")
}

#[cfg(feature = "bench_noop")]
#[cfg(feature = "target_wasmtime")]
fn run_noop() -> f32 {
    run_on_wasmtime(KERNEL_NOOP, "run_noop")
}

// target_wasmi_on_bare_metal

#[cfg(feature = "bench_regexredux")]
#[cfg(feature = "target_wasmi_on_bare_metal")]
fn run_regexredux() -> f32 {
    kernel_wasmi::run_regexredux_impl(clock_ms())
}

#[cfg(feature = "bench_coremark")]
#[cfg(feature = "target_wasmi_on_bare_metal")]
fn run_coremark() -> f32 {
    kernel_wasmi::run_coremark_impl(clock_ms())
}

#[cfg(feature = "bench_noop")]
#[cfg(feature = "target_wasmi_on_bare_metal")]
fn run_noop() -> f32 {
    kernel_wasmi::run_noop_impl(clock_ms())
}

// target_bare_metal

#[cfg(feature = "bench_regexredux")]
#[cfg(feature = "target_bare_metal")]
fn run_regexredux() -> f32 {
    kernel_regexredux::run_regexredux_impl(clock_ms())
}

fn main() {
    let target = target();

    #[cfg(feature = "bench_regexredux")]
    println!("regexredux,{},{}", target, run_regexredux());

    #[cfg(feature = "bench_coremark")]
    #[cfg(not(feature = "target_bare_metal"))]
    println!("coremark,{},{}", target, run_coremark());

    #[cfg(feature = "bench_noop")]
    #[cfg(not(feature = "target_bare_metal"))]
    println!("noop,{},{}", target, run_noop());
}
