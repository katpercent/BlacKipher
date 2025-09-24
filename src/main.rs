/*
 * Entry point for the BlacKipher Chat application.
 *
 * Responsibilities:
 *  - Initialize the sodiumoxide cryptographic library
 *  - Create demo users (the local user and a few contacts)
 *  - Build the contact list
 *  - Launch the Iced application with the Elm-style `update` and `view`
 *
 * Note: This setup is for demonstration and testing only.
 * In a real application, contacts would be dynamically managed
 * rather than hardcoded here.
 */

pub mod client; // contains user.rs and contacts.rs
pub mod ui;     // contains app.rs (UI logic)

use crate::client::contacts::Contacts;
use crate::client::user::User;
use crate::ui::app::{update, view, UI};
use iced::{application, Theme, Task};

fn main() -> iced::Result {
    /* Initialize sodiumoxide (required for crypto operations).
     * If initialization fails, the program will exit.
     */
    if sodiumoxide::init().is_err() {
        eprintln!("sodiumoxide init failed");
        std::process::exit(1);
    }

    /* Demo: create the local user */
    let me = User::new("katpercent", 4);

    /* Demo: create two additional users */
    let alice = User::new("alice", 4);
    let bob = User::new("bob", 4);
    // Uncomment for debugging key material:
    // alice.print_keys();
    // bob.print_keys();

    /* Build the contact list (excluding the current user) */
    let mut contacts = Contacts::new();
    contacts.add(alice.clone());
    contacts.add(bob.clone());

    /* Launch the Iced application.
     *
     * - Title   : "BlacKipher Chat"
     * - Update  : `update` function handles state changes
     * - View    : `view` function renders the UI
     * - Theme   : Dark mode
     * - Startup : Initializes UI with the current user and contacts
     */
    application("BlacKipher Chat", update, view)
        .theme(|_ui: &UI| Theme::Dark)
        .centered()
        .run_with(move || (UI::with_contacts(contacts, me.clone()), Task::none()))
}
