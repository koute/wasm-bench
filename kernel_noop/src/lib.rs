#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn run_noop() -> f32 {
    3.1415
}
