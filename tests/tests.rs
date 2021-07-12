use microtemplate;

// movie review graciously provided by @kg583
#[derive(microtemplate::Substitutions)]
struct Movie<'a> {
    name: &'a str,
    year: &'a str,
    category: &'a str,
    director: &'a str,
    description: &'a str,
}

const THE_BIRDS: Movie = Movie {
    name: "The Birds",
    year: "1963",
    category: "thriller",
    director: "Alfred Hitchcock",
    description: "about a swarm of birds that suddenly and violently attack the residents of a California coastal town",
};

#[test]
fn no_substitutions_found() {
    // neither of these substitutions are in THE_BIRDS
    let rendered = microtemplate::render("{movie_name} is about {things}", THE_BIRDS);

    // things that are not found are replaced with the empty string.
    assert_eq!(rendered, " is about ");
}

#[test]
fn one_substitution_found() {
    let rendered = microtemplate::render("{name} is a movie.", THE_BIRDS);

    assert_eq!(rendered, "The Birds is a movie.");
}

#[test]
fn many_substitutions_found() {
    let rendered = microtemplate::render(
        "{name}, a {year} {category} by director {director}, is {description}.",
        THE_BIRDS,
    );

    assert_eq!(rendered, "The Birds, a 1963 thriller by director Alfred Hitchcock, is about a swarm of birds that suddenly and violently attack the residents of a California coastal town.");
}

#[test]
fn substitution_at_end() {
    let rendered = microtemplate::render("Today's movie is {name}", THE_BIRDS);

    assert_eq!(rendered, "Today's movie is The Birds");
}