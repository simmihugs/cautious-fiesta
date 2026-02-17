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

are a thing as well. However despite that multiple types might
fullfill the requirement, a function returing something which is true
to the trait still can only return 1 type!

Traits can also be implemented based on former trait implementaions
i.e. if type A implements a trait B, we can say either type A also
implements trait C, or a generic implementer of type B is also
implementing trait c, so that now type A can effectively use both
traits functionlalities.

#### lifetimes

Lifetimes are basically the scope behavior, a variable starts its live
when it is initiallized and ends at the end of the scoe when its
dropped, or when it is explicitly dropped. What lifetime specifier do
is not to change the lifetime of a specific variable, but rather give
the compiler a rule how the lifetimes have to be in order for
variables to be legal in their behavior. e.g. if we return something
which is a reference, based on the input paramenters, the life time of
the input parameter must be larger or equal then the return value as
it is a reference to it. Similarly if we have multiple parameters and
it depends on some factor which of inputs is reference by the output/s
the largest life time has to be choosen.

Lifetime elision, is when the compiler can predict the lifetimes for
reference returns and the compiler does therefore not have to specify
it. e.g.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
does not need to be
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

basically there are three rules the compiler automatically applies.
1. each parameter gets its own lifetime `'a 'b 'c` etc.
2. if there is only on parameter each output parameter gets their
   lifetime i.e. `'a`
3. if there is a self in input parameters, each output parameter gets
   the self's lifetime

that means if there is one input each output just gets it lifetime and
everyting is nice and cosy. However, if there are two the compiler
does not know which parameters lifetime is the lifetime of the output
parameter --> compiletime error

#### static'

The `static'` lifetime is special as it indicates that the variable
lives for the entire life of the program. 

## Testing

Running test via cargo test from within `tests`
```shell
cargo test
```

### tests

Define test via `#[test]` in test files.

### should_panic

Use `#[should_panic]` for functions in which we expect a `panic!`.

### tests which return results

Tests can also be written by return a result which is okay <--> test
succeeds or error <--> test fails.

### commandline options

```shell
cargo test -- --help
```
reveals how to run, specific tests, ignore spefic tests, list all test
and how to test in specific ways.


```shell
cargo test -- --skip open_it
```

for instances skips the test open_it.

```shell
cargo test -- --list
```

lists all tests.

### test suite 

the tests are standard wise run in parallel, which might cause issues,
the standard shortcut is

```shell
cargo test -- --test-threads=1
```
to enforce that no parallel execution happens. E.g. tests might create 
log files and doing that in parallel might cause issues.

### function output
successfull tests capture function output, so if the function does
e.g. print to stdout, nothing is visible in test output because the
test was successful.  failed test do show the output. In order to
generally show the output, use `cargo test -- --show-output`.

### ignore tests

`#[ignore]` is a flag we can give to tests. running `cargo test` will
still run them. but specifing to ignore them will ignore them then
i.e. `cargo test -- --ignored`

### unit tests vs. integration tests
unit tests are small and test the module in isolation; Integration
test approach like a user of the code/library and execute perhabs
multiple modules.

Unit test are inside the file they test using a module `mod tests`
which is conveniently not part of the resulting
library/executable. Also unit test can access everything as they are
within the module whereas intergration test only ever see the public
parts!

### test specific integration test via filename
In order to test `tests/test_haus.rs` while ignoring test `open_it` we use
```shell
╭[simmi@simmi-ThinkPad-T440s] ~/Projects/rustbook/the_book 
╰─> cargo test --test test_haus -- --skip open_it
```

### common test code
If there is code we need/want in our test functions, we could use a
file to keep this shared code in the test suite.

## Closures

Closures can capture from their surrounding.

```rust
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
```

here the closure captures `self`, to use it for `self.most_stocked()`. 

### type annotations

```rust
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
```

closures do not need type annotations, they infere the type. however,
a closure is not generic, the first usage determines its type and
further usages are then invalid if the type are broken.

### iterators

use `iter()` to basically create a iterator, which refrences the
original to basically read from it. Use `into_iter()` to take over the
iterators origin and own it.

```rust 
let vec = vec![1, 2, 3, 4];
let even_vec: Vec<i32> = vec.iter()
    .filter(|x| **x % 2 == 0)
    .map(|&x| x)
    .collect();

println!("{:?}", vec);      // [1, 2, 3, 4] - Still exists!
println!("{:?}", even_vec); // [2, 4]

//better
let vec: Vec<i32> = vec
    .into_iter() // Consume the original
    .filter(|x| x % 2 == 0) // x is now just i32
    .collect();
```
