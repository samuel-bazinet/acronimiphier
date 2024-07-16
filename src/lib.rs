use std::collections::{HashMap, HashSet};

use acronym::Acronym;
use token::Token;

pub mod acronym;
pub mod token;

const LIMIT: usize = 10;

pub fn process(input: &str, cutoff: usize) -> String {
    let mut pairs: HashMap<(Token, Token), usize> = HashMap::new();
    let mut acronyms: HashMap<String, Acronym> = HashMap::new();
    let mut last_token = Token::default();
    let mut first_run = true;
    let mut out = String::from(input);
    let mut iteration = 0;

    // figure out how to not have the program run to infinity and beyond

    while (first_run || *pairs.values().max().unwrap_or(&0) > cutoff) && iteration < LIMIT {
        pairs = HashMap::new();
        for word in out.split(' ') {
            let trimmed_word = word
                .replace(&[',', '.', '(', ')', '\'', '"', '‘', '’', '…', '!'][..], "")
                .to_lowercase();

            if !last_token.is_empty() {
                //println!("Token: {trimmed_word}");
                if acronyms.contains_key(&trimmed_word.to_uppercase()) {
                    //println!("Found acronym {trimmed_word}");
                    let acronym = acronyms.get(&trimmed_word.to_uppercase()).unwrap();
                    let pair = (last_token, Token::Acronym(acronym.clone()));

                    pairs
                        .entry(pair)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    if trimmed_word != word.to_lowercase() {
                        last_token = Token::default();
                    } else {
                        last_token = Token::Acronym(acronym.clone());
                    }
                } else {
                    let pair = (last_token, Token::Word(trimmed_word.clone()));

                    pairs
                        .entry(pair)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    if trimmed_word != word.to_lowercase() {
                        last_token = Token::default();
                    } else {
                        last_token = Token::Word(trimmed_word);
                    }
                }
            } else {
                if !trimmed_word.is_empty() {
                    last_token = if acronyms.contains_key(&trimmed_word) {
                        Token::Acronym(acronyms.get(&trimmed_word.to_uppercase()).unwrap().clone())
                    } else {
                        Token::Word(trimmed_word)
                    };
                }
            }
        }
        first_run = false;

        let mut blacklist = HashSet::new();

        for ((pre, suf), occurence) in &pairs {
            if !blacklist.contains(&pre) && *occurence >= cutoff {
                let mut comps = pre.get_components();
                comps.extend(suf.get_components());
                let new_acronym = Acronym::from(comps);
                println!("New acronym: {}", new_acronym);
                out = out.replace(
                    &new_acronym.get_components_string(),
                    &new_acronym.generate_acronym(),
                );
                acronyms
                    .entry(new_acronym.generate_acronym())
                    .or_insert(new_acronym);
                blacklist.insert(suf);
            }
        }
        iteration += 1;
    }

    out
}
