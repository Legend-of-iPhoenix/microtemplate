pub use microtemplate_derive::Substitutions;

#[cfg(feature = "actix-web")]
use actix_web::{dev::Path, dev::Url};

pub trait Context {
    fn get_field(&self, field_name: &str) -> &str;
}

#[cfg(feature = "actix-web")]
impl Context for Path<Url> {
    fn get_field(&self, field_name: &str) -> &str {
        self.get(field_name).unwrap()
    }
}

pub fn render<C: Context>(input: &str, context: &C) -> String {
    let mut output = String::with_capacity(input.len());

    let input_bytes = input.as_bytes();
    let mut iter = input_bytes.iter().copied().enumerate();
    let mut last_index = 0;

    while let Some((index, c)) = iter.next() {
        if c == b'{' {
            output.push_str(unsafe {
                std::str::from_utf8_unchecked(input_bytes.get_unchecked(last_index..index))
            });

            while let Some((substitution_index, c)) = iter.next() {
                if c == b'}' {
                    let field_name = unsafe {
                        // +1 skips the opening {
                        std::str::from_utf8_unchecked(input_bytes.get_unchecked(index+1..substitution_index))
                    };

                    output.push_str(context.get_field(field_name));

                    // +1 skips the closing }
                    last_index = substitution_index + 1;
                    break;
                }
            }
        }
    }

    output.push_str(unsafe {
        std::str::from_utf8_unchecked(input_bytes.get_unchecked(last_index..input.len()))
    });

    output
}
