pub mod tinywasm;
pub mod wasm3;
pub mod wasmer;
pub mod wasmi_new;
pub mod wasmi_old;
pub mod wasmtime;

pub use self::tinywasm::Tinywasm;
pub use self::wasm3::Wasm3;
pub use self::wasmer::Wasmer;
pub use self::wasmi_new::WasmiNew;
pub use self::wasmi_old::WasmiOld;
pub use self::wasmtime::Wasmtime;
use crate::utils::TestFilter;
use ::wasmi_new::ModuleImportsIter;

pub trait BenchVm {
    fn name(&self) -> &'static str;
    fn test_filter(&self) -> TestFilter {
        TestFilter::default()
    }
    fn compile(&self, wasm: &[u8], imports: ModuleImportsIter);
    fn load(&self, wasm: &[u8]) -> Box<dyn BenchRuntime>;
}

pub trait BenchRuntime {
    fn call(&mut self, input: i64);
}
