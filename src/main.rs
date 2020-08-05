use std::io;
use std::io::Read;
use std::iter::repeat;

fn character_replace((index, c): (usize, char)) -> Option<String> {
    if index == 0 {
        Some(c.to_uppercase().to_string())
    } else {
        match c {
            // n @ '0'..='9' => Some(n.to_string()),
            'b' | 'f' | 'p' | 'v' => Some('1'.to_string()),
            'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => Some('2'.to_string()),
            'd' | 't' => Some('3'.to_string()),
            'l' => Some('4'.to_string()),
            'm' | 'n' => Some('5'.to_string()),
            'r' => Some('6'.to_string()),
            _ => None,
        }
    }
}

fn soundex_word(input: &str) -> String {
    let mut last_letter: Option<String> = None;
    let mut trim_mode = true;
    input
        .chars()
        .filter_map(|c| {
            if trim_mode && c.is_alphabetic() {
                trim_mode = false;
            }
            if trim_mode {
                None
            } else {
                Some(c)
            }
        })
        .enumerate()
        .filter_map(character_replace)
        .filter_map(|s| {
            if last_letter.as_ref() == Some(&s) {
                return None;
            }
            last_letter = Some(s.clone());
            Some(s)
        })
        .chain(repeat('0'.to_string()))
        .take(4)
        .collect()
}

fn soundex_sentence(input: &str) -> String {
    input
        .split(' ')
        .map(soundex_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let result = soundex_sentence(&buffer);
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::soundex_sentence;

    #[test]
    fn dumb_test() {
        assert_eq!(soundex_sentence("Sarah"), "S600");
        assert_eq!(soundex_sentence("Sarah Connor"), "S600 C560");
        assert_eq!(soundex_sentence("123Sarah Conn23452or"), "S600 C560");
    }
}
