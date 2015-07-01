extern crate unicode_segmentation;
#[macro_use] extern crate log;

mod grammar;
mod item;
mod item_table;
mod token;

use item::Item;
use item_table::ItemTable;

use unicode_segmentation::UnicodeSegmentation;

pub use token::{Token, Terminal, NonTerminal};
pub use grammar::{Grammar, Rule};

pub fn build_items<'a>(grammar: &'a Grammar, input: &str) -> ItemTable<'a> {
    let mut s = ItemTable::new(grammar, input.len());

    let chars = UnicodeSegmentation::graphemes(input, true).chain(Some("\0").into_iter());

    for (char_index, current_char) in chars.enumerate() {
        debug!("-----> {} matching {}", char_index, current_char);
        let mut item_index = 0;
        while item_index < s.table[char_index].len() {
            let item = s.table[char_index][item_index];
            let next_item = grammar.rules[item.rule].tokens.get(item.next);
            debug!("[{}, {}] :: {} || {:?}", char_index, item_index, item.render(&grammar), next_item);
            match next_item {
                Some(&NonTerminal(token)) => s.predict(char_index, token),
                Some(&Terminal(token)) => s.scan(item, char_index, current_char, token),
                None => s.complete(item, char_index),
            }
            item_index += 1;
        }
    }

    return s;
}

pub fn matching_items(s: &ItemTable) -> Vec<Item> {
    if let Some(items) = s.table.last() {
       items.iter().filter(|item| {
           let rule = &s.grammar.rules[item.rule];
           rule.name == s.grammar.starting_rule && item.next >= rule.tokens.len() && item.start == 0
       }).map(Clone::clone).collect()
    } else {
        Vec::new()
    }
}
