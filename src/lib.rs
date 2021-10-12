#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::missing_panics_doc
)]

use phf::phf_map;

const MIRROR_CHARS: phf::Map<char, char> = phf_map! {
    '/' => '\\',
    '\\' => '/',
    '<' => '>',
    '>' => '<',
    '(' => ')',
    ')' => '(',
    '[' => ']',
    ']' => '[',
    '{' => '}',
    '}' => '{',
};

fn mirror_line<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut mirrored = String::with_capacity(s.len());
    for c in s.chars().rev() {
        if MIRROR_CHARS.contains_key(&c) {
            mirrored.push(MIRROR_CHARS[&c]);
        } else {
            mirrored.push(c);
        }
    }

    mirrored
}

pub fn mirror_text<S: AsRef<str>>(text: Vec<S>) -> Vec<String> {
    let mut mirrored = Vec::with_capacity(text.len());
    let max_length = text.iter().map(|line| line.as_ref().len()).max().unwrap();
    for line in text {
        let padding = max_length - line.as_ref().len();
        mirrored.push(" ".repeat(padding) + &mirror_line(line));
    }

    mirrored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_simple_line() {
        assert_eq!(mirror_line("asd"), "dsa");
    }

    #[test]
    fn mirror_line_with_mirror_characters_1() {
        assert_eq!(mirror_line(r"{[(<\/>)]}"), r"{[(<\/>)]}");
    }

    #[test]
    fn mirror_line_with_mixed_characters() {
        assert_eq!(mirror_line(r"| {e}  |  /     \"), r"/     \  |  {e} |");
    }

    #[test]
    fn mirror_multiline_text_simple() {
        assert_eq!(mirror_text(vec!["a", "bc"]), vec![" a", "cb"]);
    }

    #[test]
    fn mirror_multiline_text_complex() {
        let cow: Vec<&str> = r" _______
< hello >
 -------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||"
            .split("\n")
            .collect();

        let anticow: Vec<&str> = r"                    _______ 
                   < olleh >
                    ------- 
            ^__^   /        
    _______/(oo)  /         
/\/(       /(__)            
   | w----||                
   ||     ||                "
            .split("\n")
            .collect();

        assert_eq!(mirror_text(cow), anticow);
    }
}
