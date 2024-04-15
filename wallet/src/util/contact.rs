//
//  Copyright 2024 Ram Flux, LLC.
//

use fnv::FnvHasher;
use secp256k1::PublicKey;
use std::hash::{Hash, Hasher};

// def Contact type
#[derive(Clone)]
pub struct Contact {
    public_key: PublicKey,
    nickname: String,
    note: String,
}

// def address book type
#[derive(Default, Clone)]
pub struct ContactList {
    contacts: Vec<Contact>,
}

// Consistent hash structure
pub struct ConsistentHash {
    replicas: usize,
    ring: Vec<(u64, PublicKey)>,
}

impl ConsistentHash {
    pub fn new(replicas: usize) -> Self {
        ConsistentHash {
            replicas,
            ring: Vec::new(),
        }
    }

    fn add_contact(&mut self, contact: &Contact) {
        for i in 0..self.replicas {
            let mut hasher = FnvHasher::default();
            contact.public_key.serialize().hash(&mut hasher);
            i.hash(&mut hasher);
            let hash = hasher.finish();
            self.ring.push((hash, contact.public_key));
        }
        self.ring.sort_by_key(|&(hash, _)| hash);
    }

    fn remove_contact(&mut self, public_key: &PublicKey) {
        self.ring.retain(|&(_, pk)| pk != *public_key);
    }

    fn update_contact(&mut self, old_contact: &Contact, new_contact: &Contact) {
        self.remove_contact(&old_contact.public_key);
        self.add_contact(new_contact);
    }

    fn get_contact(&self, key: &PublicKey) -> Option<PublicKey> {
        let mut hasher = FnvHasher::default();
        key.serialize().hash(&mut hasher);
        let hash = hasher.finish();
        for &(h, pk) in &self.ring {
            if h >= hash {
                return Some(pk);
            }
        }
        self.ring.first().map(|&(_, pk)| pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_hash() {
        let mut consistent_hash = ConsistentHash::new(100);

        let mut contact_list = ContactList {
            contacts: vec![
                Contact {
                    public_key: PublicKey::from_slice(&[1; 33]).unwrap(),
                    nickname: "Alice".to_string(),
                    note: "My best friend".to_string(),
                },
                Contact {
                    public_key: PublicKey::from_slice(&[2; 33]).unwrap(),
                    nickname: "Bob".to_string(),
                    note: "A good friend".to_string(),
                },
                Contact {
                    public_key: PublicKey::from_slice(&[3; 33]).unwrap(),
                    nickname: "Carol".to_string(),
                    note: "A new friend".to_string(),
                },
            ],
        };

        for contact in &contact_list.contacts {
            consistent_hash.add_contact(contact);
        }

        let key = PublicKey::from_slice(&[0; 33]).unwrap();
        if let Some(contact_key) = consistent_hash.get_contact(&key) {
            println!("Key {:?} is mapped to contact {:?}", key, contact_key);
        } else {
            println!("No contact found for key {:?}", key);
        }

        // del a contact
        let removed_contact = contact_list.contacts.remove(1);
        consistent_hash.remove_contact(&removed_contact.public_key);

        // update a contact
        let old_contact = &contact_list.contacts[0];
        let new_contact = Contact {
            public_key: old_contact.public_key,
            nickname: "Alice (updated)".to_string(),
            note: "My best friend (updated)".to_string(),
        };
        consistent_hash.update_contact(old_contact, &new_contact);
        contact_list.contacts[0] = new_contact;

        // find the contact for the key again
        if let Some(contact_key) = consistent_hash.get_contact(&key) {
            println!(
                "After update, key {:?} is mapped to contact {:?}",
                key, contact_key
            );
        } else {
            println!("After update, no contact found for key {:?}", key);
        }
    }
}
