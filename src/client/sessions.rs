/*
 * This module defines the `Session` system, which is responsible
 * for persisting encrypted conversations with contacts.
 *
 * Each conversation is identified by the recipient's username
 * and contains a list of `StoredMessage` entries.
 *
 * Messages are serialized to and from JSON, enabling persistence
 * across application runs.
 */

use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/*
 * Represents a single encrypted message that is stored in a session.
 *
 * Fields:
 *  - `ciphertext`   : The encrypted payload of the message
 *  - `ephemeral_pk` : The sender's ephemeral public key (as raw bytes)
 *  - `nonce`        : The nonce used during encryption
 *  - `log`          : A human-readable log of the encryption/decryption process
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct StoredMessage {
    pub ciphertext: Vec<u8>,
    pub ephemeral_pk: Vec<u8>,
    pub nonce: Vec<u8>,
    pub log: String,
}

/*
 * Represents all conversations in a given session.
 *
 * A `Session` maps each recipient username to a vector of `StoredMessage`s,
 * effectively storing a per-contact message history.
 */
#[derive(Serialize, Deserialize, Default)]
pub struct Session {
    /* Key = recipient username, Value = list of messages with that recipient */
    pub conversations: HashMap<String, Vec<StoredMessage>>,
}

impl Session {
    /*
     * Loads a session from a JSON file located at `path`.
     *
     * If the file does not exist, or if deserialization fails,
     * a new empty session is returned instead.
     */
    pub fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Session::default()
        }
    }

    /*
     * Saves the current session state to a JSON file at `path`.
     *
     * The session is serialized using pretty JSON formatting for readability.
     */
    pub fn save(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    /*
     * Adds a new message to the session under the given recipient.
     *
     * Parameters:
     *  - `recipient`   : The username of the message recipient
     *  - `ciphertext`  : The encrypted message payload
     *  - `ephemeral_pk`: The sender's ephemeral public key
     *  - `nonce`       : The encryption nonce
     *  - `log`         : A detailed log string for debugging/inspection
     *
     * If the recipient does not already exist in the session,
     * a new conversation entry will be created automatically.
     */
    pub fn add_message(
        &mut self,
        recipient: &str,
        ciphertext: Vec<u8>,
        ephemeral_pk: box_::PublicKey,
        nonce: box_::Nonce,
        log: String,
    ) {
        let entry = self.conversations.entry(recipient.to_string()).or_default();
        entry.push(StoredMessage {
            ciphertext,
            ephemeral_pk: ephemeral_pk.as_ref().to_vec(),
            nonce: nonce.0.to_vec(),
            log,
        });
    }

    /*
     * Retrieves all stored messages for the given recipient.
     *
     * Returns `Some(&Vec<StoredMessage>)` if a conversation exists,
     * or `None` if no conversation has been recorded with that recipient.
     */
    pub fn get_messages(&self, recipient: &str) -> Option<&Vec<StoredMessage>> {
        self.conversations.get(recipient)
    }
}
