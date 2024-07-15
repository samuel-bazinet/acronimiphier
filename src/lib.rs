use std::collections::HashMap;

use acronym::Acronym;
use token::Token;

pub mod acronym;
pub mod token;

pub fn process(input: &str, cutoff: usize) -> String {
    let mut pairs: HashMap<(Token, Token), usize> = HashMap::new();
    let mut acronyms: HashMap<String, Acronym> = HashMap::new();
    let mut last_token = Token::default();
    let mut first_run = true;
    let mut out = String::from(input);

    while first_run || *pairs.values().max().unwrap_or(&0) > cutoff {
        for word in input.split(' ') {
            let trimmed_word = word.replace(&[',', '.', '(', ')', '\'', '"', '‘', '’', '…'][..], "");

            if !last_token.is_empty() {
                if acronyms.contains_key(&trimmed_word) {
                    let acronym = acronyms.get(&trimmed_word).unwrap();
                    let pair = (last_token, Token::Acronym(acronym.clone()));

                    pairs
                        .entry(pair)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    if trimmed_word != word {
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
                    if trimmed_word != word {
                        last_token = Token::default();
                    } else {
                        last_token = Token::Word(trimmed_word);
                    }
                }
            } else {
                last_token = Token::Word(trimmed_word);
            }
        }
        first_run = false;

        for ((pre, suf), occurence) in &pairs {
            if *occurence >= cutoff {
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
            }
        }
        pairs = HashMap::new();
    }
    
    out
}
