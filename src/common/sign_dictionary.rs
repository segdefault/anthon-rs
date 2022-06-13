use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::common::Sign;

const NEW_SIGN_NAME: &str = "New Sign ";

#[derive(Serialize, Deserialize)]
pub struct SignDictionary {
    signs: BTreeMap<String, Sign>,
}

impl SignDictionary {
    pub fn signs(&self) -> &BTreeMap<String, Sign> {
        &self.signs
    }

    pub fn signs_mut(&mut self) -> &mut BTreeMap<String, Sign> {
        &mut self.signs
    }

    pub fn next_valid_name(&self) -> String {
        let mut sign_name = String::from(NEW_SIGN_NAME);
        let mut i = 1;

        while self.signs.contains_key(&sign_name) {
            sign_name = String::from(NEW_SIGN_NAME) + &i.to_string();
            i += 1;
        }

        sign_name
    }
}

pub struct IndexedSign<'a> {
    pub index: usize,
    pub name: &'a String,
    pub sign: &'a Sign,
}

impl SignDictionary {
    pub fn find_similar(&self, sign: &Sign) -> Option<IndexedSign> {
        for (index, (name, other)) in self.signs.iter().enumerate() {
            if other == sign {
                return Some(IndexedSign {
                    index,
                    name,
                    sign: other,
                });
            }
        }

        None
    }

    pub fn get_by_index(&self, index: usize) -> Option<(&String, &Sign)> {
        for (i, sign) in self.signs.iter().enumerate() {
            if i == index {
                return Some(sign);
            }
        }

        None
    }
}

impl From<BTreeMap<String, Sign>> for SignDictionary {
    fn from(map: BTreeMap<String, Sign>) -> Self {
        SignDictionary { signs: map }
    }
}

impl From<SignDictionary> for BTreeMap<String, Sign> {
    fn from(dict: SignDictionary) -> Self {
        dict.signs
    }
}
