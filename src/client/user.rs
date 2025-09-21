use sodiumoxide::crypto::{box_, sign};
use hex;

pub struct User {
    pub username: String,

    pub identity_pk: box_::PublicKey,
    pub identity_sk: box_::SecretKey,

    pub signed_pre_pk: box_::PublicKey,
    pub signed_pre_sk: box_::SecretKey,
    // Optionnel: la signature Ed25519 sur la signed prekey
    pub signed_pre_sig: sign::Signature,

    pub one_time_prekeys: Vec<(box_::PublicKey, box_::SecretKey)>,
}

impl User {
    pub fn new(username: &str, num_prekeys: usize) -> Self {
        // 1. Identity key (long terme) : Ed25519
        let (id_pk, id_sk) = sign::gen_keypair();

        // 2. Signed PreKey (X25519) signée avec l'identity secret key
        let (spk_pk, spk_sk) = box_::gen_keypair();
        let sig = sign::sign_detached(&spk_pk.0, &id_sk);

        // 3. Générer plusieurs One-Time PreKeys (X25519)
        let mut ot_prekeys = Vec::new();
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


    pub fn print_keys(&self) {
        println!("User: {}", self.username);
        println!("  Identity Public Key : {}", hex::encode(self.identity_pk.0));
        println!("  Identity Secret Key : {}", hex::encode(self.identity_sk.0));
        println!("  Signed Pre Public   : {}", hex::encode(self.signed_pre_pk.0));
        println!("  Signed Pre Secret   : {}", hex::encode(self.signed_pre_sk.0));
        println!("  Signature on SPK    : {}", hex::encode(self.signed_pre_sig.0));

        for (i, (pk, sk)) in self.one_time_prekeys.iter().enumerate() {
            println!("  One-Time PreKey #{i} (pub): {}", hex::encode(pk.0));
            println!("  One-Time PreKey #{i} (priv): {}", hex::encode(sk.0));
        }
    }
}

