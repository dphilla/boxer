
<div align="right">
  <details>
    <summary >🌐 Language</summary>
    <div>
      <div align="center">
        <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=en">English</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=zh-CN">简体中文</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=zh-TW">繁體中文</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=ja">日本語</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=ko">한국어</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=hi">हिन्दी</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=th">ไทย</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=fr">Français</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=de">Deutsch</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=es">Español</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=it">Italiano</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=ru">Русский</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=pt">Português</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=nl">Nederlands</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=pl">Polski</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=ar">العربية</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=fa">فارسی</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=tr">Türkçe</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=vi">Tiếng Việt</a>
        | <a href="https://openaitx.github.io/view.html?user=dphilla&project=boxer&lang=id">Bahasa Indonesia</a>
      </div>
    </div>
  </details>
</div>

# Boxer

<p align="center">
  <p align="center">
    <a href="https://en.wikipedia.org/wiki/Mia_St._John" target="_blank">
      <img src="https://user-images.githubusercontent.com/20820229/164059786-8d082b44-59d6-431a-adf4-993116c8d492.png" alt="Mia St John" width="300"             height="400">
    </a>
  </p>
</p>

**This project is under rapid development**

# Boxer

## Overview
Boxer is an open-source project designed to reimagine cloud computing by transitioning from traditional container technology to WebAssembly (Wasm) based distributions, known as "Boxes" or "Wasm-Boxes".

## Motivation
While containers have been a cornerstone of cloud computing, providing isolated environments for applications, they come with significant drawbacks. They often result in larger, less efficient deployments with slower startup times and rely heavily on the underlying operating system/kernel for security, which can introduce vulnerabilities and platform lock-in.

## Solution: Boxer
Boxer offers a new solution by providing tooling for converting existing containerized workloads and definitions into near-universally deployable Wasm distributions. These "Boxes" offer environments comparable to those provided by containers but are vastly more efficient thanks to the lightweight, sandboxed execution capabilities of WebAssembly.

<p align="center">
  <p align="center">
    <a href="https://boxer.dev" target="_blank">
      <img src="https://github.com/dphilla/wasm-vfs/assets/20820229/4b0309d8-c8ae-427a-8af3-67857a8eebf3" alt="Container and Box" width="400"             height="400">
    </a>
  </p>
</p>


### Key Advantages of WebAssembly:
- **Reduced Overhead:** Wasm's compact binary instruction format ensures lightweight execution, drastically cutting down the overhead seen in traditional containers.
- **Enhanced Performance:** The efficiency of Wasm leads to improved performance and smaller, more efficient deployments, perfectly suited for cloud computing.
- **Superior Security:** Wasm's memory-safe, isolated execution environment provides a higher level of security, independent of the operating system.
- **Run (almost) Everywhere** Wasm can run a anywhere there is a WebAssembly runtime, including browsers, servers, and embedded devices; existing runtimes are available on for a [wide variety of architectures.](https://github.com/appcypher/awesome-wasm-runtimes)

## Example

### Dockerfile ➡ (Wasm)Box

For for this file:

```Dockerfile
FROM scratch
RUN mkdir -p /app
COPY a.out /app
WORKDIR /app
CMD ["/app/a.out"]
```

run  `box build -f Dockerfile`

This will bring in libc interaces (as imports) and create the necessary FS state, thus declaritivey creating a ready-to-run Wasm binary.
