use std::cell::RefCell;
use std::collections::HashMap;
use std::string::String;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        
        // Sanitise
        let mut lower = String::new();
        for c in paragraph.to_lowercase().chars() {
            if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() {
                lower.push(c.clone());
            }
            if c.is_ascii_punctuation() {
                lower.push(' ');
            }
        }
        
        // Iterate
        let mut map: HashMap<String, RefCell<u32>> = HashMap::new();
        for w in lower.split_ascii_whitespace() {
            if let Some(i) = map.get(&String::from(w)) {
                let mut ct = i.borrow_mut();
                *ct = *ct + 1;
            } else {
                map.insert(String::from(w), RefCell::new(1));
            }
        }
        
        // Find max
        let mut s: String = String::from("");
        let mut len: u32 = 0;
        for (w, count) in map.into_iter() {
            if banned.iter().any(|i| i == &w) {
                continue;
            }
            if *count.borrow() > len {
                len = *count.borrow();
                s = String::from(w);
            }
        }
        
        s
    }
}