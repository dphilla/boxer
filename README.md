# WasmBoxer

<p align="center">
  <p align="center">
    <a href="https://en.wikipedia.org/wiki/Mia_St._John" target="_blank">
      <img src="https://user-images.githubusercontent.com/20820229/164059786-8d082b44-59d6-431a-adf4-993116c8d492.png" alt="Mia St John" width="300"             height="400">
    </a>
  </p>
</p>


## Dockerfile -> Wasm Binary

### Example

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
