| Test Name                     | rwasm   | wasmi-old | wasmi-new.eager.checked | wasmi-new.lazy.checked | tinywasm | wasm3.eager | wasm3.lazy | stitch  | wasmtime.cranelift | wasmer.cranelift | wasmer.singlepass |
|-------------------------------|---------|-----------|-------------------------|------------------------|----------|-------------|------------|---------|--------------------|------------------|-------------------|
| execute/counter/1000000       | 30.278  | 13.985    | 6.6394                  | 6.4825                 | 27.997   | 4.2883      | 3.8483     | 2.9161  | 0.30994            | 0.51477          | 0.64885           |
| execute/fib.recursive/30      | 154.70  | 98.024    | 73.927                  | 73.167                 | 166.11   | 48.936      | 49.301     | 33.524  | 5.6011             | 5.6588           | 6.531             |
| execute/fib.iterative/2000000 | 103.40  | 52.275    | 22.182                  | 20.614                 | 95.830   | 15.478      | 15.280     | 11.786  | 1.5394             | 1.4041           | 3.0661            |
| execute/fib.tailrec/1000000   | 60.112  | 33.198    | 25.634                  | 23.265                 | N/A      | N/A         | N/A        | N/A     | 1.7907             | N/A              | N/A               |
| execute/primes/1000           | 3.1154  | 1.7089    | 1.2124                  | 1.2764                 | 3.4232   | 0.59493     | 0.58889    | 0.48282 | 0.16574            | 0.17132          | 0.18877           |
| execute/matmul/200            | 1057.2  | 434.84    | 280.24                  | 276.73                 | 1.0609   | 139.52      | 188.58     | 126.07  | 22.422             | 22.698           | 50.296            |
| execute/argon2/1              | 269.15  | 123.38    | 102.48                  | 110.35                 | 272.07   | 51.934      | 42.190     | N/A     | 2.6754             | 2.9670           | 6.7561            |
| execute/bulk-ops/5000         | 0.84105 | 0.64199   | 0.56537                 | 0.50612                | 0.89037  | 0.49535     | 0.47220    | 0.54306 | 0.53507            | 0.45383          | 0.51198           |
| execute/rwasm-jit             | 7.8943  | 4.2415    | 2.7080                  | 2.7188                 | N/A      | N/A         | N/A        | N/A     | N/A                | 0.73034          | 0.89411           |

