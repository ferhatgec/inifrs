// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
// inifrs - fast .ini file parser crate
//
// github.com/ferhatgec/inifrs
// github.com/ferhatgec/inif.py
// github.com/ferhatgec/inifrs

pub enum InifTokens {
    CategoryStart,
    CategoryEnd,
    Eq,
    Comment,
    Undef
}

impl InifTokens {
    pub fn val(self) -> char {
        use InifTokens::{*};

        match self {
            CategoryStart => '[',
            CategoryEnd   => ']',
            Eq            => '=',
            Comment       => ';',
            Undef         => ' '
            // _             => unreachable!("undefined token")
        }
    }

    pub fn to(ch: char) -> InifTokens {
        use InifTokens::{*};

        match ch {
            '[' => CategoryStart,
            ']' => CategoryEnd,
            '=' => Eq,
            ';' => Comment,
            _   => Undef
        }
    }
}

pub struct InifCategory {
    category_name: String,
    key_value: std::collections::HashMap<String, String>
}

pub struct Inif {
    init: Vec<InifCategory>,
    category_start: bool,
    category_data: bool,
    key_data: bool,

    category_name: String,
    key: String,
    data: String
}

impl Default for Inif {
    fn default() -> Self {
        Inif {
            init          : vec![],
            category_start: false,
            category_data : false,
            key_data      : false,
            category_name : "".to_string(),
            key           : "".to_string(),
            data          : "".to_string()
        }
    }
}

impl Inif {
    pub fn parse(&mut self, file_data: String) {
        use crate::InifTokens::{*};

        for ch in file_data.chars() {
            if self.key_data {
                if ch != '\n' {
                    self.data.push(ch);
                    continue;
                }

                self.key = self.key.trim().to_string();

                if self.init.len() > 0 && self.init.last().unwrap().category_name == self.category_name {
                    self.init.last_mut().unwrap().key_value.insert(self.key.clone(), self.data.clone());
                } else {
                    let mut val: std::collections::HashMap<String, String>
                        = std::collections::HashMap::new();

                    val.insert(self.key.clone(), self.data.clone());

                    self.init.push(InifCategory {
                        category_name: self.category_name.clone(),
                        key_value    : val
                    });
                }

                self.key_data = false;
                self.key.clear(); self.data.clear();

                continue;
            }

            if self.category_start {
                if ch != InifTokens::val(CategoryEnd) {
                    self.category_name.push(ch);
                    continue;
                }

                self.category_start = false;
                self.category_data  = true;

                continue;
            }

            match InifTokens::to(ch) {
                CategoryStart => {
                    if !self.category_name.is_empty() {
                        self.category_name.clear();
                    }

                    self.category_start = true;
                },
                Eq => {
                    if self.category_data && !self.key.is_empty() {
                        self.key_data = true;
                    }
                },
                Undef | _ => {
                    if self.category_data && ch != ' ' {
                        self.key.push(ch);
                    }
                }
            }
        }
    }

    pub fn get(&self, category: String, key: String) -> String {
        for val in &self.init {
            if val.category_name == category {
                for store in &val.key_value {
                    if *store.0 == key {
                        return store.1.clone();
                    }
                }
            }
        }

        "\"\"".to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hmm() {
        use crate::{*};

        let file_data = std::fs::read_to_string("example.ini").unwrap();
        let mut val = Inif::default();

        val.parse(format!("{}\n", file_data));

        println!("{}\n{}\n{}",
                 val.get("name".to_string(), "name".to_string()),
                 val.get("projects".to_string(), "test".to_string()),
                 val.get("not".to_string(), "found".to_string()));
    }
}
