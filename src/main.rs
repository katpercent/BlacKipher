pub mod client;
use sodiumoxide::init;
use crate::client::user::User;

fn main() {
    // initialisation nécessaire pour sodiumoxide
    if init().is_err() {
        eprintln!("sodiumoxide init failed");
        std::process::exit(1);
    }

    // Exemple : créer 2 users
    let alice = User::new("alice");
    let bob = User::new("bob");

    // affiche les clés en hex
    alice.print_keys();
    bob.print_keys();

    // Si tu veux stocker/serialiser, tu as maintenant Vec<u8> pour chaque clé.
}
