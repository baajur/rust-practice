///
/// solution for anagram in long text problem, cracking the coding interview, sixth edition, page 70
///
use std::collections::HashMap;

///
/// checks if two vecs are equal
/// for the purposes of this algorithm, this is effectively constant time
/// it is dependent on the length of the alphabet chosen, not on the input size
///
fn is_anagram(chars: &Vec<usize>, pattern: &Vec<usize>) -> bool {
    chars
        .iter()
        .eq(pattern.iter())
}

///
/// maps a str into a character count vector, given the alphabet
/// ex "aabb", given alphabet "abcde", returns [2, 2, 0, 0, 0]
/// rust chars vs string slices are *hard*!
///
fn create_pattern_hash(pattern: &str, alphabet: &HashMap<char, usize>) -> Vec<usize> {
    let mut result: Vec<usize> = vec![0; alphabet.len()];
    for ch in pattern.chars() {
        let ind: usize = match alphabet.get(&ch) {
            Some(&x) => x,
            None => panic!(format!("character {} not in alphabet", ch))
        };
        result[ind] += 1;
    }
    result
}

///
/// given a pattern, text, and alphabet of possible characters
/// return the start position of all anagrams of the pattern
/// alphabet defaults to english lower case letters if not specified
///
/// this algorithm iterates through text in windows of length pattern.len()
/// it maps the pattern to a tokenized character count vector like "aab" -> [2, 1, 0...]
/// and maps the first window to its corresponding character count vector
/// then moves through text, updating the char count vector in place
/// it is effectively O(|text|*|alphabet|) time, but |alphabet| is considered constant
///
pub fn solve(pattern: &str, text: &str, alphabet: Option<&HashMap<char, usize>>) -> Vec<usize> {
    if pattern.len() == 0 || text.len() == 0 {
        return Vec::new();
    } else if pattern.len() > text.len() {
        return Vec::new();
    }
    let mut default_alphabet: HashMap<char, usize> = HashMap::new();
    let alphabet: &HashMap<char, usize> = {
        match alphabet {
            Some(x) => x,
            None => {
                let letters = "abcdefghijklmnopqrstuvwxyz";
                letters
                    .char_indices()
                    .for_each(|(i, c)| { default_alphabet.insert(c, i); });
                &default_alphabet
            }
        }
    };
    let pattern_hash = create_pattern_hash(pattern, alphabet);
    let mut section: Vec<usize> = create_pattern_hash(&text[0..pattern.len() - 1], alphabet);
    let mut result: Vec<usize> = Vec::new();
    let text_vec: Vec<char> = text.chars().collect();
    for (i, window) in text_vec.windows(pattern.len()).enumerate() {
        section[alphabet[&window.last().unwrap()]] += 1;
        let pattern_match = is_anagram(&section, &pattern_hash);
        if pattern_match {
            result.push(i);
        }
        section[alphabet[&window[0]]] -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{create_pattern_hash, solve};

    #[test]
    fn test_create_pattern_hash() {
        let alphabet: HashMap<char, usize> = {
            let string = "abcd".to_string();
            string
                .char_indices()
                .map(|(i, c)| (c, i))
                .collect()
        };
        let string = "aad";
        let result = create_pattern_hash(string, &alphabet);
        assert_eq!(result, vec![2usize, 0, 0, 1]);
    }

    #[test]
    fn zero_items() {
        let pattern = String::from("");
        let text = String::from("");
        let result = solve(&pattern, &text, None);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn simple_example() {
        let pattern = String::from("aa");
        let text = String::from("ababaac");
        let alphabet: HashMap<char, usize> = "abc"
            .char_indices()
            .map(|(i, c)| (c, i))
            .collect();
        let result = solve(&pattern, &text, Option::from(&alphabet));
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn example() {
        let pattern = String::from("abbc");
        let text = String::from("cbabadcbbabbcbabaabccbabc");
        let result = solve(&pattern, &text, None);
        assert_eq!(result, vec![0usize, 6, 9, 11, 12, 20, 21])
    }
}
