# microtemplate
A fast, microscopic helper crate for runtime string interpolation.

## Design Goals
 - **Very lightweight**: I want `microtemplate` to do exactly one thing. There's no reason to include more feature-rich/large libraries like `regex`/`handlebars`/`tinytemplate` if all you need is string interpolation.
 - - **Zero extra runtime sub-dependencies**: The only dependencies used are for the derive macro, which is handled at compile time.
 - **Fast**: I'm interested in making this library as fast as I can make it. It's extremely fast right now, but I do not believe it is perfect.
 - **Simple**.

## Quickstart
Add `microtemplate` to your dependencies:
```toml
[dependencies]
microtemplate = "1.0.1"
```

Usage example (from the tests):
```rust
use microtemplate::{Substitutions, render};

// This derive allows microtemplate to use the struct as substitutions.
#[derive(Substitutions)]
struct Movie<'a> {
    name: &'a str, // automatically replaces "{name}" in a template
    description: &'a str,
}

fn main() {
    let the_birds = Movie {
        name: "The Birds",
        description: "a swarm of birds that suddenly and violently attack the residents of a California coastal town",
    };

    // the template string here can be generated whenever- in this example it
    // is known at compile time but that does not matter.
    let rendered = render("{name} is a movie about {description}.", the_birds);

    // The Birds is a movie about a swarm of birds that suddenly and violently
    // attack the residents of a California coastal town.
    println!("{}", rendered);

    // note that if a substitution is indicated in the template string but it is
    // not found in the struct passed, it is replaced with an empty string ("")
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

If you have any bug reports, improvements, or feature requests, please [make an issue](https://github.com/Legend-of-iPhoenix/microtemplate/issues/new)