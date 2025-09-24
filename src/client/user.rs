/*
 * This module defines the `User` struct, which models a participant
 * in an end-to-end encrypted system inspired by the Signal protocol.
 *
 * Each user has:
 *  - An identity key pair (Ed25519, long-term identity)
 *  - A signed pre-key pair (X25519, signed by the identity key)
 *  - A set of one-time pre-keys (X25519, used for initial sessions)
 *
 * It also provides helper methods for:
 *  - Key generation and printing
 *  - Verifying a peer's signed pre-key
 *  - Encrypting and decrypting messages with detailed logs
 *
 * Note: This is a **simplified educational version** of Signal's
 * X3DH + Double Ratchet protocol. It demonstrates the concepts
 * without full forward secrecy guarantees.
 */

use sodiumoxide::crypto::{box_, sign};
use hex;

/*
 * Represents a user with cryptographic identity and pre-keys.
 *
 * Fields:
 *  - `username`        : Human-readable name of the user
 *  - `identity_pk/sk`  : Long-term Ed25519 identity key pair
 *  - `signed_pre_pk/sk`: Signed pre-key pair (X25519), validated with `identity_sk`
 *  - `signed_pre_sig`  : Signature of the signed pre-key, produced by `identity_sk`
 *  - `one_time_prekeys`: Collection of one-time pre-keys (X25519)
 */
#[derive(Clone)]
pub struct User {
    pub username: String,

    pub identity_pk: sign::PublicKey,
    pub identity_sk: sign::SecretKey,

    pub signed_pre_pk: box_::PublicKey,
    pub signed_pre_sk: box_::SecretKey,
    pub signed_pre_sig: sign::Signature,

    pub one_time_prekeys: Vec<(box_::PublicKey, box_::SecretKey)>,
}

impl User {
    /*
     * Creates a new user with the given username.
     *
     * Steps:
     *  1. Generate an identity key pair (Ed25519)
     *  2. Generate a signed pre-key pair (X25519)
     *     - The public part is signed with the identity secret key
     *  3. Generate `num_prekeys` one-time pre-keys (X25519)
     */
    pub fn new(username: &str, num_prekeys: usize) -> Self {
        let (id_pk, id_sk) = sign::gen_keypair();

        let (spk_pk, spk_sk) = box_::gen_keypair();
        let sig = sign::sign_detached(spk_pk.as_ref(), &id_sk);

        let mut ot_prekeys = Vec::with_capacity(num_prekeys);
        for _ in 0..num_prekeys {
            ot_prekeys.push(box_::gen_keypair());
        }

        Self {
            username: username.to_string(),
            identity_pk: id_pk,
            identity_sk: id_sk,
            signed_pre_pk: spk_pk,
            signed_pre_sk: spk_sk,
            signed_pre_sig: sig,
            one_time_prekeys: ot_prekeys,
        }
    }

    /*
     * Prints the user's cryptographic keys in hexadecimal form.
     *
     * This is intended for debugging and educational inspection,
     * not for production use.
     */
    pub fn print_keys(&self) {
        println!("User: {}", self.username);
        println!("  Identity Public Key : {}", hex::encode(self.identity_pk.as_ref()));
        println!("  Identity Secret Key : {}", hex::encode(self.identity_sk.as_ref()));
        println!("  Signed Pre Public   : {}", hex::encode(self.signed_pre_pk.as_ref()));
        println!("  Signed Pre Secret   : {}", hex::encode(self.signed_pre_sk.as_ref()));
        println!("  Signature on SPK    : {}", hex::encode(self.signed_pre_sig.as_ref()));
        for (i, (pk, sk)) in self.one_time_prekeys.iter().enumerate() {
            println!("  One-Time PreKey #{i} (pub): {}", hex::encode(pk.as_ref()));
            println!("  One-Time PreKey #{i} (priv): {}", hex::encode(sk.as_ref()));
        }
    }

    /* Returns the username of the user */
    pub fn username(&self) -> &str {
        &self.username
    }

    /*
     * Verifies that a peer's signed pre-key is valid.
     *
     * This checks that the peer's `signed_pre_pk` was signed by
     * their identity secret key using Ed25519.
     */
    pub fn verify_peer_spk(peer: &User) -> bool {
        sign::verify_detached(&peer.signed_pre_sig, peer.signed_pre_pk.as_ref(), &peer.identity_pk)
    }

    /*
     * Encrypts a message to a peer with logging.
     *
     * Steps:
     *  0. Verify the peer's signed pre-key signature
     *  1. Generate an ephemeral key pair (used only for this message)
     *  2. Perform Diffie-Hellman: DH(ephemeral, peer.SPK)
     *  3. Generate a random nonce
     *  4. Encrypt the plaintext using the derived shared secret
     *  5. Produce a human-readable log with all details
     *
     * Returns:
     *  - Ephemeral public key
     *  - Nonce
     *  - Ciphertext
     *  - Debug log (String)
     */
    pub fn encrypt_message_with_logs(
        &self,
        peer: &User,
        plaintext: &str,
    ) -> (box_::PublicKey, box_::Nonce, Vec<u8>, String) {
        let spk_ok = Self::verify_peer_spk(peer);

        let (ephemeral_pk, ephemeral_sk) = box_::gen_keypair();
        let shared = box_::precompute(&peer.signed_pre_pk, &ephemeral_sk);

        let nonce = box_::gen_nonce();
        let ciphertext = box_::seal_precomputed(plaintext.as_bytes(), &nonce, &shared);

        let log = format!(
            concat!(
                "== log ==\n",
                "Sender: {}\nReceiver: {}\n",
                "Verify(peer.SPK signed by peer.ID) = {}\n",
                "Ephemeral PK: {}\n",
                "DH(ephemeral, peer.SPK): precomputed ({} bytes)\n",
                "Nonce: {}\n",
                "Ciphertext: {}\n"
            ),
            self.username,
            peer.username,
            spk_ok,
            hex::encode(ephemeral_pk.as_ref()),
            shared.0.len(),
            hex::encode(nonce.0),
            hex::encode(&ciphertext),
        );

        (ephemeral_pk, nonce, ciphertext, log)
    }

    /*
     * Decrypts a message from a peer with logging.
     *
     * Steps:
     *  1. Perform Diffie-Hellman: DH(sender.ephemeral, self.SPK)
     *  2. Decrypt the ciphertext with the derived shared secret
     *  3. Produce a human-readable log of the process
     *
     * Returns:
     *  - `Some((plaintext, log))` if decryption succeeds
     *  - `None` if decryption or UTF-8 decoding fails
     */
    pub fn decrypt_message_with_logs(
        &self,
        sender_ephemeral_pk: &box_::PublicKey,
        nonce: &box_::Nonce,
        ciphertext: &[u8],
        sender_name: &str,
    ) -> Option<(String, String)> {
        let shared = box_::precompute(sender_ephemeral_pk, &self.signed_pre_sk);

        let pt = box_::open_precomputed(ciphertext, nonce, &shared).ok()?;
        let plaintext = String::from_utf8(pt).ok()?;

        let log = format!(
            concat!(
                "== log (recv) ==\n",
                "Receiver: {}\nSender: {}\n",
                "DH(sender.ephemeral, self.SPK): precomputed ({} bytes)\n",
                "Nonce: {}\n",
                "Ciphertext: {}\n",
                "Plaintext: {}\n"
            ),
            self.username,
            sender_name,
            shared.0.len(),
            hex::encode(nonce.0),
            hex::encode(ciphertext),
            plaintext
        );

        Some((plaintext, log))
    }
}
