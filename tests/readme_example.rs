use microtemplate::{Substitutions, render};

// This derive allows microtemplate to use the struct as substitutions.
#[derive(Substitutions)]
struct Movie<'a> {
    name: &'a str, // automatically replaces "{name}" in a template
    description: &'a str,
}

#[test]
fn readme_example() {
    let the_birds = Movie {
        name: "The Birds",
        description: "a swarm of birds that suddenly and violently attack the residents of a California coastal town",
    };

    // the template string here can be generated whenever- in this example it is known at compile time but that does not matter.
    let rendered = render("{name} is a movie about {description}.", the_birds);

    assert_eq!(rendered, "The Birds is a movie about a swarm of birds that suddenly and violently attack the residents of a California coastal town.");

    // note that if a substitution is indicated in the template string but it is not found in the struct passed, it is replaced with an empty string ("")
}