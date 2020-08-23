pub mod wordlist;
use wordlist::WORDS;

extern crate rand;
use crate::rand::{thread_rng, Rng, distributions::Uniform};

pub mod ipv4;

pub const DIVIDER: &str = ".";

pub fn generate_epid3() -> String {
    generate_epid_size(3)
}

pub fn generate_epid4() -> String {
    generate_epid_size(4)
}

pub fn generate_epid6() -> String {
    generate_epid_size(6)
}

fn generate_epid_size(num_words: usize) -> String {
    thread_rng()
        .sample_iter(Uniform::new(0, WORDS.len()))
        .take(num_words)
        .map(|i| WORDS[i].into())
        .collect::<Vec<String>>()
        .join(DIVIDER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epid3_has_two_dividers() {
        let id = generate_epid3();
        assert_eq!(id.matches(DIVIDER).count(), 2);
    }

    #[test]
    fn epid4_has_three_dividers() {
        let id = generate_epid4();
        assert_eq!(id.matches(DIVIDER).count(), 3);
    }
}
