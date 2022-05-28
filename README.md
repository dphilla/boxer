<p align="center">
  <p align="center">
    <a href="https://rails.spinup.dev" target="_blank">
      <img src="https://user-images.githubusercontent.com/20820229/164059786-8d082b44-59d6-431a-adf4-993116c8d492.png" alt="Mia St John" width="300"             height="400">
    </a>
  </p>
</p>

# wasm-boxer

## Compile your project to WebAssembly in a single step

### Description - WIP

This repo is a collection of efforts related to the `wasm-boxer` project. The aim of this project is to create a unified build tool for compiling many language runtimes to WebAssembly (Wasm). This work draws on several WebAssembly tooling projects to make the endeavor possible.


### What is a Wasm Box?

If you compile your project's language runtime, its dependencies, and application code to a WebAssembly module, what then? You may still need to do things like: make a network call over udp, tcp or https, access a filesystem, or communicate other processes running on a host. To acheive this, while leveraging the sandboxing security features and portability of WebAssembly, the concept of using other WebAssembly modules to act as, or even replace, some of these other components. 

This group of modules can be understood as a **"Wasm Box"** -- an isolated group of system-like resources that allow your project to run on almost any host.

For ex.  -- a Python FAST API app + wasi-vfs, wasi-vn, and WASI-ipc. 

See more: wasm component model, nanoprocess model, etc.

### Related

A Wasm Box is an implemenation of the [Component Model](https://github.com/WebAssembly/component-model), influenced by the [Nanoprocess Model](https://bytecodealliance.org/articles/1-year-update). 

Related and underlying fundamentals:

- [WASI](https://github.com/WebAssembly/WASI)
- [Module Linking](https://github.com/WebAssembly/module-linking)
- [Interface Types](https://github.com/WebAssembly/interface-types/blob/main/proposals/interface-types/Explainer.md)

