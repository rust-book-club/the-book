use std::collections::HashSet;

pub mod anslatortray {

    pub fn anslatetray(ingstray: &str) -> String {

        // how can I make this a module-level const?
        // how can I `use` outside this module, but not have to use `super` below?
        let vowels: super::HashSet<char> = super::HashSet::from(['a', 'e', 'i', 'o', 'u', 'y']);

        if ingstray.starts_with(|c: char| vowels.contains(&c)) {
            format!("{}-hay", ingstray)
        } else {
            match ingstray.find(|c: char| vowels.contains(&c)) {
                None => ingstray.to_string(),
                Some(index) => format!("{}-{}ay", &ingstray[index..], &ingstray[0..index])
            }
        }
    }
}