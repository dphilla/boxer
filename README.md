# WasmBoxer

<p align="center">
  <p align="center">
    <a href="https://en.wikipedia.org/wiki/Mia_St._John" target="_blank">
      <img src="https://user-images.githubusercontent.com/20820229/164059786-8d082b44-59d6-431a-adf4-993116c8d492.png" alt="Mia St John" width="300"             height="400">
    </a>
  </p>
</p>


## Build your project to run in WebAssembly in a single step.

### Description

WasmBoxer is a unified build tool for implementing many language runtimes to [WebAssembly (Wasm)](https://webassembly.org). This work draws on several WebAssembly tooling projects to make the endeavor possible.

### Who is this for?

Anying building a project in Python or Ruby who wants to deploy a Wasm/WASI distribution, rather than another deployment strategy, like baremetal, a VM, or a container.

### A Wasm Box?

If you compile your project's language runtime, its dependencies, and application code to a WebAssembly module, what then? You may still need to do things like make a network, access a filesystem, or communicate with other processes running on a host. To acheive this, while leveraging the sandboxing security features and portability of WebAssembly, the concept of using other WebAssembly modules to act as, replace, or provide interfaces with some of these other components.

This grouping of modules can be understood as a **Wasm Box** -- an isolated collection of resources that allow your project to run on almost any host.

For ex. -- a Python FAST API / Rails App + wasi-vfs, wasi-vn.

### Related

A Wasm Box is an implemenation of the [Component Model](https://github.com/WebAssembly/component-model), influenced by the [Nanoprocess Model](https://bytecodealliance.org/articles/1-year-update).

Related and underlying fundamentals:

- [WASI](https://github.com/WebAssembly/WASI)
- [Module Linking](https://github.com/WebAssembly/module-linking)
- [Interface Types](https://github.com/WebAssembly/interface-types/blob/main/proposals/interface-types/Explainer.md)

