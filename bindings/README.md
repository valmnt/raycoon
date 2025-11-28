# Raycoon – Bindings

Raycoon exposes a minimal C FFI so it can be used from other languages.  
The idea is to keep the engine purely in Rust while making it portable elsewhere.

- **Possible bindings:** any language capable of calling a C library can interface with it.
- **Officially maintained binding:** **Python** (a `ctypes` wrapper in `bindings/python`).
- **Other languages:** they may use the Python binding as inspiration, but are not supported by default.
