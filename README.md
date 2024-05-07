# Wasmi Benchmarking Suite

This includes execution and compilation benchmarks for the Wasmi interpreter and other Wasm runtimes.

## Runtimes

The following Wasm runtimes and configurations are included.

| Runtime | ID | Configurations | Note |
|:-------:|:---------------|:-----|:---|
| Wasmi v0.31 | `wasmi-v0.31` | | |
| Wasmi v0.32 | `wasmi-v0.32` | `eager`, `eager.unchecked`, `lazy`, `lazy-translation` | |
| Tinywasm | `tinywasm` | | Included because it was kinda simple. |
| Wasm3 | `wasm3` | `eager`, `lazy` | Generally accepted to be the fastest Wasm interpreter. |
| Wasmtime | `wasmtime` | `cranelift`, `winch` | Winch only works on `x86` platforms. |
| Wasmer | `wasmer` | `cranelift`, `singlepass` | |

### Configuration Explanation

- `eager`: All function bodies are compiled immediately.
- `eager.unchecked`: Function bodies are compiled eagerly but not validated.
- `lazy`: Function bodies are only compiled on their first actual use.
- `lazy-translation`: Function bodies are lazily compiled but eagerly validated.
- `cranelift`: The runtime uses the Cranelift code generator.
- `wasmtime.winch`: Wasmtime's JIT optimized for fast start-up times.
- `wasmer.singlepass`: Wasmer's JIT optimized for fast start-up times.

## Usage

Run all benchmarks via:

```
cargo bench
```

**Note:** compilation might take some minutes since we are compiling a lot of Wasm runtimes with very high optimization settings.

Filter benchmarks via

- `compile`: for compilation benchmarks.
- `execute`: for execution benchmarks.
- The runtime `ID`, e.g. `wasmi-v0.31` or `wasm3`.
- The runtime configuration on top of the runtime `ID`, e.g. `wasmi-v0.32.lazy`.
- Single test names, e.g. `counter` (execute) or `ffmpeg` (compile)

Examples

Run all runtimes on the `counter` execution benchmark test case:

```
cargo bench execute/counter
```

Run all Wasm3 test cases with its eager compilation configuration:

```
cargo bench wasm3.eager
```

## Test Cases

The Wasmi benchmarking test suite provides the following test cases:

| Mode | Test Case | Notes |
|:----:|:---------:|:------|
| `execute` | | |
| | `counter` | Simple loop that counts a single local up to some number. |
| | `fib.recursive` | Recursive fibonacci calculation. Call-intense workload. |
| | `fib.iterative` | Iterative fibonacci calculation. Compute intense workload. |
| | `fib.tailrec` | Tail-call based fibonacci calculation. |
| | `primes` | Calculates all primes until some number. Uses linear memory for storing known primes. |
| | `matmul` | Naive matrix multiplication implementation. Makes heavy use of linear memory and floats. |
| `compile` | | |
| | `bz2` | Medium-sized compression library with huge function bodies. |
| | `pulldown-cmark` | Medium-sized markdown renderer. |
| | `spidermonkey` | The firefox Javascript execution engine. (large) |
| | `ffmpeg` | Huge multimedia library. |
| | `coremark` | CoreMark benchmarking compilation. (kinda small) |
