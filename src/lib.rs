pub use microtemplate_derive::Substitutions;

pub trait Context {
    fn get_field(&self, field_name: &str) -> &str;
}

pub fn render<T: Context>(string: &str, context: T) -> String {
    let mut new_string = String::with_capacity(string.len());
    let mut last_index = 0;
    let mut iter = string.as_bytes().iter().copied().enumerate();
    while let Some((mut x_index, x)) = iter.next() {
        if x == b'{' {
            new_string.push_str(unsafe {
                std::str::from_utf8_unchecked(string.as_bytes().get_unchecked(last_index..x_index))
            });

            for (y_index, y) in &mut iter {
                if y == b'}' {
                    let field_str = context.get_field(unsafe {
                        std::str::from_utf8_unchecked(string.as_bytes().get_unchecked(last_index..x_index))
                    });
                    new_string.push_str(field_str);
                    x_index = y_index + 1;
                    break;
                }
            }
            last_index = x_index;
        }
    }
    new_string.push_str(unsafe {
        std::str::from_utf8_unchecked(string.as_bytes().get_unchecked(last_index..string.len()))
    });
    new_string
}