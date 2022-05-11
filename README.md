<img src="https://user-images.githubusercontent.com/20820229/164059786-8d082b44-59d6-431a-adf4-993116c8d492.png" width="300" height="400" style="margin-left:600px"></img>

# wasm-boxer

## Compile your project to WebAssembly in a single step.

`wbx script.rb`

`wbx script.rb --web`

`wbx script.rb --runtime wasmtime`

Ex. a Rails app:

`wbx .` 


### Box up your project to run in almost any environment.

`Default `wbx` compiles AOT to Wasm (no runtime needed -- HOPEFULLY). The output is a .wasm file that can run on any [WebAssembly Runtime](https://github.com/appcypher/awesome-wasm-runtimes), or be Ahead-of-Time compiled.

show benchmarks compared to containers vms: start up times, size, native vs wasm excution, etc. tons and tons of good visualizations.
