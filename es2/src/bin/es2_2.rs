fn slugify(s: &str) -> String {
    let mut slug = s.chars()
        .flat_map(|c| c.to_lowercase())
        .map(conv)
        .collect::<String>();

    while slug.contains("--") {
        slug = slug.replace("--", "-");
    }

    if slug != "-" {
        slug = slug.trim_end_matches('-').to_string();
    }

    slug
}

fn conv(c: char) -> char {
    if c.is_ascii_lowercase() || c.is_ascii_digit() {
        c
    } else if let Some(i) = SUBS_I.char_indices().position(|(_, ch)| ch == c) {
        SUBS_O.chars().nth(i).unwrap_or('-')
    } else {
        '-'
    }
}

const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <text>", args[0]);
        return;
    }
    let text = &args[1];
    let slug = slugify(text);
    println!("Slug: {}", slug);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("à"), "a");
        assert_eq!(slugify("A"), "a");
        assert_eq!(slugify("§"), "-");
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("Rust is awesome!"), "rust-is-awesome");
        assert_eq!(slugify("  Leading and trailing spaces  "), "-leading-and-trailing-spaces");
        assert_eq!(slugify("Multiple   spaces"), "multiple-spaces");
        assert_eq!(slugify("Special characters: !@#$%^&*()"), "special-characters");
        assert_eq!(slugify("Numbers 12345"), "numbers-12345");
        assert_eq!(slugify("Mixed 123 and !@#"), "mixed-123-and");
        assert_eq!(slugify("Unicode: 你好"), "unicode");
        assert_eq!(slugify("Emoji: 😊🚀"), "emoji");
        assert_eq!(slugify("Doppelgänger"), "doppelganger");
        assert_eq!(slugify(""), "");
        assert_eq!(slugify("   "), "-");
        assert_eq!(slugify("$$$"), "-");
        assert_eq!(slugify("ῶ"), "-");
        assert_eq!(slugify("àèéòùì"), "aeeoui");
    }
}