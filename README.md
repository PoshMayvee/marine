# Marine

![marine version on crates.io](https://img.shields.io/crates/v/marine?color=green&style=flat-square)

Marine is a modern general purpose Wasm runtime based on the [component model](https://github.com/WebAssembly/component-model) capable of running multi-module Wasm applications, aka services, with [interface-types](https://github.com/WebAssembly/interface-types) and a [shared-nothing linking](https://training.linuxfoundation.org/blog/how-and-why-to-link-webassembly-modules/) scheme. This execution model is well suited for a variety of scenarios and especially applicable to implementations following the [entity component system](https://en.wikipedia.org/wiki/Entity_component_system) (ECS) pattern or plugin-based architectures.

Fluence peers, such as Fluence [Rust node](https://github.com/fluencelabs/fluence), include Marine to execute the hosted Wasm services composed with [Aqua](https://github.com/fluencelabs/aqua).

## Motivational example

To illustrate the capabilities of Marine, let's have a look at a multi-module Wasm service as implemented in [this](./examples/motivational-example) example.

`cd` into the `examples/motivational-example` directory and have a look at the  `shrek/src/main.rs` file: 

```rust
// examples/motivational-example/shrek/src/main.rs
use marine_rs_sdk::marine;

fn main() {}

#[marine]
pub fn greeting(name: &str) -> Vec<String> {
    let donkey_greeting = donkey::greeting(name);         // 1
    let shrek_greeting = format!("Shrek: hi, {}", name);  // 2
    
    vec![shrek_greeting, donkey_greeting]                 
}

mod donkey {                                               // 3
    use super::*;

    #[marine]
    #[link(wasm_import_module = "donkey")]                 // 4
    extern "C" {
        pub fn greeting(name: &str) -> String;
    }
}
```

In this Marine (Wasm) module (and namespace) `shrek`, we declare a function `greeting` that creates a `donkey_greeting` (1) from the `donkey` module's (3)`greeting` function, which itself is dependent on importing the `donkey` **Wasm module** with Rust's FFI [`link`](https://doc.rust-lang.org/nomicon/ffi.html) (4) from `examples/motivational-example/donkey/src/main.rs` (see below).

```rust
// examples/motivational-example/donkey/src/main.rs
use marine_rs_sdk::marine;

fn main() {}

#[marine]
pub fn greeting(name: &str) -> String {
    format!("Donkey: hi, {}", name)
}
```

In summary, our example is comprised of two independent Wasm modules, `shrek` and `donkey`, and illustrates how to link one module into another one, i.e., use the `donkey` module in the `shrek` module. Please note that the `shrek` module is called a *facade* module following the [facade pattern]((https://en.wikipedia.org/wiki/Facade_pattern)) and there can only be one *facade* module per service. 


Make sure you have the Marine tools [installed](https://fluence.dev/docs/marine-book/quick-start/setting-up-the-development-environment) and compile the `donkey` and `shrek`, respectively, which we can do with the `build.sh` script:

```bash
$> ./build.sh
```

which creates two independent Wasm modules that are placed in the `artifact` directory:

```bash
$> ls artifacts
donkey.wasm    shrek.wasm
```

Now that we have our modules, we can explore them with the Marine REPL. Note that we use the  `Config.toml` file to help out the REPL by providing the module location and names. Once we got the REPL up and running, we can interact with both modules and, as expected, the `shrek` module is successfully able to access the `donkey` module's exposed functions.

```bash
$> marine mrepl Config.toml
...
1> interfaces
Loaded modules interface:

shrek:
  fn greeting(name: string) -> []string
donkey:
  fn greeting(name: string) -> string

2> call donkey greeting "no link module"
result: String("Donkey: hi, no link module")
 elapsed time: 42.985µs

3> call donkey greeting "facade with link module"
result: String("Donkey: hi, facade with link module")
 elapsed time: 39.25µs

4> q
```

Looks like everything is in order and the modules are ready for [deployment to the network](https://fluence.dev/docs/build/quick-start/hosted-services/#deploying-a-wasm-module-to-the-network) and [composition with Aqua](https://fluence.dev/docs/build/quick-start/service-composition-and-reuse/).

## Repository structure

- **[REPL](./tools/repl)** - REPL intended to test Marine Wasm modules
- **[CLI](./tools/cli)** - CLI intended to build and extract some info from Marine Wasm modules
- **[fluence-app-service](./fluence-app-service)** - Fluence Application Service layer provides basic API for service running
- **[fluence-faas](./fluence-faas)** - Fluence FaaS layer provides host closures, IT<->JSON conversion, logger, config handling and other
- **[runtime](./runtime)** - runtime layer provides basic functionality for loading, unloading and calling modules
- **[marine-js](./marine-js)** - web runtime layer aim to run Marine in browser
- **[examples](./examples)** - several Marine examples used mostly for tests
- **[it-generator](./crates/it-generator)** - generator of IT
- **[it-interfaces](./crates/it-interfaces)** - handy structure for interface types handling
- **[it-json-serde](./crates/it-json-serde)** - crate for conversion between IT and JSON
- **[min-it-version](./crates/min-it-version)** - keeps minimal supported versions of IT and SDK by runtime
- **[module-info-parser](./crates/module-info-parser)** - parser of module manifest and sdk version
- **[module-interface](./crates/module-interface)** - parser of module IT
- **[utils](./crates/utils)** - some utility functions and consts

## References

- [Documentation](https://fluence.dev/docs/marine-book/introduction)
- [Examples](https://github.com/fluencelabs/examples/tree/main/marine-examples)
- [Quickstart](https://fluence.dev/docs/marine-book/quick-start/)
