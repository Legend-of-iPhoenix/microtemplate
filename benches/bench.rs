#![feature(test)]

use microtemplate;

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

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

    #[bench]
    fn bench_no_templates(b: &mut Bencher) {
        b.iter(|| {
            black_box(microtemplate::render("No templates found in this fifty-character string.", THE_BIRDS));
        });
    }

    #[bench]
    fn bench_one_template_found(b: &mut Bencher) {
        b.iter(|| {
            black_box(microtemplate::render("The name of the movie is {name}", THE_BIRDS));
        })
    }

    #[bench]
    fn bench_many_templates_found(b: &mut Bencher) {
        b.iter(|| {
            black_box(microtemplate::render("{name}, a {year} {category} by director {director}, is {description}.", THE_BIRDS));
        });
    }
}