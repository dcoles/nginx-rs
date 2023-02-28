# nginx-rs

[![crates.io](https://img.shields.io/crates/v/nginx-rs.svg)](https://crates.io/crates/nginx-rs)
[![MIT License](https://img.shields.io/crates/l/nginx-rs.svg)](LICENSE)

> **Note**
> [Feb 2023] I haven't had a chance to work on this recently, but you might be interested in
> Cloudflare's blog post [*ROFL with a LOL: rewriting an NGINX module in Rust*](https://blog.cloudflare.com/rust-nginx-module/), which takes some inspiration from this module.

A framework for writing Nginx modules in pure Rust.

This module is in early stages. It lacks documentation and the API is still quite unstable.
But it can be used to write simple request handlers for content or access control.

## Building Modules

Building modules requires a checkout of the Nginx sources
[configured for building dynamic modules](https://www.nginx.com/blog/compiling-dynamic-modules-nginx-plus/):

```bash
export NGINX_DIR=/path/to/nginx
cd "${NGINX_DIR}"
auto/configure --with-compat
```

Once Nginx is configured, you can then build your module:

```bash
cd /path/to/module
cargo build --release
```

The resulting `.so` in `target/release` can then be loaded using the
[`load_module` directive](https://nginx.org/en/docs/ngx_core_module.html#load_module).

## Examples

- [hello_world](/examples/hello_world) — Demonstrations access control and content handlers

## Licence

This project is licensed under the terms of the [MIT license](LICENSE).
