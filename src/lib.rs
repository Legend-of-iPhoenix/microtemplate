use std::str::Chars;

pub use microtemplate_derive::Substitutions;

pub trait Context {
    fn get_field(&self, field_name: &str) -> &str;
}

#[inline(always)] // need 4 speed
fn resolve_substitution<'a, C: Context>(iter: &mut Chars, context: &'a C) -> &'a str {
    let mut field_name = String::with_capacity(4); // reasonable lower bound
    while let Some(c) = iter.next() {
        match c {
            '}' => return context.get_field(&field_name),
            _ => field_name.push(c),
        }
    }

    ""
}

pub fn render<C: Context>(input: &str, context: C) -> String {
    let first = input.find('{'); // handle the case where there's no substitution
    if let Some(first) = first {
        let mut output = String::from(&input[0..first]);
        output.reserve(input.len() - first);

        let mut iter = input[first..].chars();
        while let Some(c) = iter.next() {
            match c {
                '{' => output.push_str(resolve_substitution(&mut iter, &context)),
                _ => output.push(c),
            }
        }

        output
    } else {
        input.to_string() // not sure if I like this.
    }
}
