# Refresher in Rust

## Unit Structs

Units structs can be leveraged to have application states embedded in
type level.  That means we create a generic system type which depends
on a generic parameter T, and then implement behavior or traits based
on the specific versions of the system based on the actual type.

E.g. we have a type which represends the nuclear powerplant system

```rust
struct PowerPlant<Mode> {
	mode: Mode,
	...
}
```

and then say there is a mode `Critical`, `Idle`, `Off`, etc. Then in
this example only `PowerPlant<Idle>` can be shutdown gracefully; only
`Powerplant<Critical>` can ever do certain things related to critical
behavior like meltdown crisis operations or crisis shutdown etc.

## Packages, Crates, Modules

Rust has a number of features that allow you to manage your code’s
organization, including which details are exposed, which details are
private, and what names are in each scope in your programs. These
features, sometimes collectively referred to as the module system,
include:

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy
of paths Paths: A way of naming an item, such as a struct, function,
or module

### Libraries

```shell
cargo new kuchen --lib
```

### Pub

In `struct`s every field needs to be set to public in order to be
accessible from outside of the module. In enums however, making the
enum public makes the entire enums parameters public as well.

### use

For functions use `use` to access the module defining a function to
then use it like `module::function`, but for types rather use 
`use path::module::Type` to then use it like `Type`. Exception to 
this rule are types with the same name, where we either again import
the module in order to then `module1::Type` and `module2::Type`. 
Alternatively we could do `use path::module1::Type as module1Type` 
and then use `module1Type` as type in order to avoid conflicts.

### error handling
either use `panic!` for unrecoverable errors like access to index of
containers which do not exist i.e. `let vec = vec![1,2,3]; vec[99]` in
order to fail early rather then have undefined behavior or error
checks every where. In order to have a save way of dealing with
potential errorneous behaviour provide defensive apis as an
alternative, which do use `Option`s and or `Result` which enforce a
handling via the type system.

### generics

Rust uses monomorphization on generics at compile time <--> generic
code gets compiled into specific versions e.g. `Option<T>` into
`Option<i32>` and `Option<f64>`. 

#### traits

```rust
trait Summarize {
	fn summarize(&self) -> String;
}

struct Article {...}

impl Summarize for Article {...}

fn summarize_it(text: &impl Summarize) {...}
//vs.
fn summarize_it<T: Summarize>(text: &T) {...}
```

here the two ways to ensure the type does implement the trait are
still different. The `<>` way is more concise, but diencourages for
everyday code I gues because it is noisier, but it would allow to
explicitly define the specific type. the Trait bound (`<>`) has the
advantage to allow us to define that both types are the same if a
function usese multiple parameters. e.g.

```rust
fn func<T: Summarize>(t1: &T, t2: &T) {...}

//vs

fn func(t1: &impl Summarize, t2: &impl Summarize) {...}
```

where the second only ensures we do imple `Summarize` but not if t1
and t2 are different types.

If a type has multiple traits it adheres to, using a + helps
i.e. `&(impl Debug + Summarize)` or `<T: Debug + Summarize>` and if
the traits are to many the option to write everything between function
head and body using `where` exit i.e.
```rust
fn summarize_it_further_<T>(t: &T)
where
    T: Summary + Hello,
{
    println!("{}", t.summarize());
    t.hello();
}
```

Finally, traits on return types
```rust
fn create_article() -> impl Summary + Hello {
    Article {
        text: "hello".to_string(),
    }
}
```

are a thing as well.
