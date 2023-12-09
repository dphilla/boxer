# WasmBoxer

<p align="center">
  <p align="center">
    <a href="https://en.wikipedia.org/wiki/Mia_St._John" target="_blank">
      <img src="https://user-images.githubusercontent.com/20820229/164059786-8d082b44-59d6-431a-adf4-993116c8d492.png" alt="Mia St John" width="300"             height="400">
    </a>
  </p>
</p>

# WasmBoxer

## Overview
WasmBoxer is an open-source project designed to reimagine cloud computing by transitioning from traditional container technology to WebAssembly (Wasm) based distributions, known as "Boxes" or "Wasm-Boxes".

## Motivation
While containers have been a cornerstone of cloud computing, providing isolated environments for applications, they come with significant drawbacks. They often result in larger, less efficient deployments with slower startup times and rely heavily on the underlying operating system/kernel for security, which can introduce vulnerabilities and platform lock-in.

## Solution: WasmBoxer
WasmBoxer offers a new solution by providing tooling for converting existing containerized workloads and definitions into near-universally deployable Wasm distributions. These "Boxes" offer environments comparable to those provided by containers but are vastly more efficient thanks to the lightweight, sandboxed execution capabilities of WebAssembly.

### Key Advantages of WebAssembly:
- **Reduced Overhead:** Wasm's compact binary instruction format ensures lightweight execution, drastically cutting down the overhead seen in traditional containers.
- **Enhanced Performance:** The efficiency of Wasm leads to improved performance and smaller, more efficient deployments, perfectly suited for cloud computing.
- **Superior Security:** Wasm's memory-safe, isolated execution environment provides a higher level of security, independent of the operating system.
- **Run (almost) Everywhere** Wasm can run a anywhere there is a WebAssembly runtime, including browsers, servers, and embedded devices; existing runtimes are available on for a [wide variety of architectures.](https://github.com/appcypher/awesome-wasm-runtimes)

## Example

### Dockerfile -> Wasm Binary

For for this file:

```Dockerfile
FROM ubuntu:latest
RUN mkdir -p /app
COPY a.out /app
WORKDIR /app
CMD ["/app/a.out"]
```

run  `wbx build -f Dockerfile`

This will bring in libc interaces (as imports) and create the necessary FS state, thus declaritivey creating a ready-to-run Wasm binary.
