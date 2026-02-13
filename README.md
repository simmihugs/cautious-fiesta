# Refresher in Rust

## Unit Structs

Units structs can be leveraged to have application states embedded in type level.
That means we create a generic system type which depends on a generic parameter T,
and then implement behavior or traits based on the specific versions of the system
based on the actual type.

E.g. we have a type which represends the nuclear powerplant system

```rust
struct PowerPlant<Mode> {
	mode: Mode,
	...
}
```

and then say there is a mode `Critical`, `Idle`, `Off`, etc. Then in this example 
only `PowerPlant<Idle>` can be shutdown gracefully; only `Powerplant<Critical>` 
can ever do certain things related to critical behavior like meltdown crisis 
operations or crisis shutdown etc.

## Packages, Crates, Modules

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

### Libraries

```shell
cargo new kuchen --lib
```

test
