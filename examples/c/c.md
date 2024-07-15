# Example Box with a C program

Note: This is not emcc. Read about the differences [here]().

## Via CLI

Just put `box` in front your compiler command. This then just uses the standard compiler (gcc, clang, rust, go etc.), but first intercepts some of the code that would make [syscalls](https://en.wikipedia.org/wiki/System_call) on a native system, so that your code work with WebAssembly, in a box.

`box gcc <file> ...`

then COPY the resluting binary via your Dockerfile

## Multi-stage builds

_Build exactly like you would in a mutli-stage container declaration_

E.g.

```
# Stage 1: Build the application
FROM gcc:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application - **this builds the golang to WebAssemblly, with the proper interception of system code in for use in a Wasm Box**
RUN gcc -o myapp main.c

# Stage 2: Create the final image
FROM alpine:latest

# Install necessary packages (if any)
RUN apk --no-cache add libc6-compat

# Set the working directory
WORKDIR /root/

# Copy the binary from the builder stage
COPY --from=builder /app/myapp .

# Expose the necessary port (if applicable)
EXPOSE 8080

# Command to run the application
CMD ["./myapp"]

```

## Why?

Many reasons. Mostly, because we don't want to (yet) attempt to have a new architecture added to LLVM, or other toolchains, and we certainly don't want to force you to use a non-standard fork or custom compiler.

Technical Explanation:

 - Wasm Compilation, Syscalls, Shared libs, etc.
 - Do we want the same C ABI?
 - The debate

