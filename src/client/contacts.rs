/*
 * This module defines the `Contacts` structure,
 * which acts as an in-memory address book for managing `User` objects.
 *
 * Features:
 *  - Add or remove users
 *  - Search for users by username
 *  - List all stored users
 *
 * Note: This implementation is intentionally simple
 * and does not enforce uniqueness of usernames.
 */

use crate::client::user::User;

/*
 * Represents a collection of contacts (users).
 *
 * Internally, this is a wrapper around a `Vec<User>`,
 * but it provides convenience methods for common operations.
 */
#[derive(Default)]
pub struct Contacts {
    /* The list of users currently stored in this contact list */
    pub users: Vec<User>,
}

impl Contacts {
    /* Creates a new, empty `Contacts` list */
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    /*
     * Adds a new user to the contact list.
     *
     * Warning: this does not prevent duplicate usernames.
     */
    pub fn add(&mut self, user: User) {
        self.users.push(user);
    }

    /*
     * Removes a user from the contact list by username.
     *
     * If multiple users share the same username,
     * all of them will be removed.
     */
    pub fn remove(&mut self, username: &str) {
        self.users.retain(|u| u.username != username);
    }

    /*
     * Finds a user by username.
     *
     * Returns:
     *  - `Some(&User)` if a match is found
     *  - `None` if no user matches the provided username
     */
    pub fn find(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }

    /*
     * Prints the list of all usernames to stdout.
     *
     * Useful for debugging or quick inspection.
     */
    pub fn list(&self) {
        for u in &self.users {
            println!("- {}", u.username);
        }
    }

    /*
     * Gets a reference to a user by username.
     *
     * This is essentially the same as `find()`,
     * but demonstrates the use of the `User::username()` accessor.
     */
    pub fn get(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username() == username)
    }
}
