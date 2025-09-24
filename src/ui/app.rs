/*
 * This module defines the graphical user interface (GUI) layer
 * using the Iced framework. It follows the Elm architecture:
 *
 *  - `UI` struct  : holds application state (contacts, current user, session, etc.)
 *  - `Message`    : represents all possible user interactions/events
 *  - `update()`   : handles logic/state transitions
 *  - `view()`     : renders the UI based on the current state
 *
 * The UI also integrates the cryptographic layer:
 * messages are encrypted before being stored in a session
 * and decrypted before being displayed in the chat window.
 */

use crate::client::contacts::Contacts;
use crate::client::sessions::Session;
use crate::client::user::User;
use iced::border::{Border, Radius};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column, Row};
use iced::{color, Alignment, Background, Color, Element, Length, Theme};
use sodiumoxide::crypto::box_;

/*
 * Holds all application state required by the GUI.
 *
 * Fields:
 *  - `input_value`      : Current text inside the message input field
 *  - `contacts`         : Contact list (other users)
 *  - `selected_contact` : Currently selected contact for conversation
 *  - `current_user`     : The active user of this client
 *  - `session`          : Persistent conversations, stored on disk
 */
pub struct UI {
    input_value: String,
    pub contacts: Contacts,
    selected_contact: Option<String>,
    pub current_user: User,
    pub session: Session,
}

impl UI {
    /*
     * Creates a new `UI` state initialized with the provided contacts
     * and current user. It automatically attempts to load any existing
     * session data from `session.json`.
     */
    pub fn with_contacts(contacts: Contacts, current_user: User) -> Self {
        let session = Session::load("session.json");
        Self {
            input_value: String::new(),
            contacts,
            selected_contact: None,
            current_user,
            session,
        }
    }
}

/*
 * Represents all possible messages/events that can occur in the UI.
 *
 * - `InputChanged`: Fired when the user types in the input field
 * - `Send`        : Fired when the user presses "Enter" or clicks "Send"
 * - `SelectContact`: Fired when the user selects a contact from the list
 */
#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Send,
    SelectContact(String),
}

/*
 * The update function (Elm-style).
 *
 * It applies state transitions based on the received message:
 *  - Handles text input updates
 *  - Encrypts and persists messages on "Send"
 *  - Changes the active conversation on "SelectContact"
 */
pub fn update(ui: &mut UI, message: Message) {
    match message {
        Message::InputChanged(value) => ui.input_value = value,
        Message::Send => {
            if let Some(name) = &ui.selected_contact {
                let text = ui.input_value.trim();
                if text.is_empty() {
                    return;
                }
                if let Some(recipient) = ui.contacts.get(name) {
                    // Encrypt the message (produces ciphertext + logs)
                    let (epk, nonce, ciphertext, send_log) =
                        ui.current_user.encrypt_message_with_logs(recipient, text);

                    // Store it persistently under the recipient's conversation
                    ui.session.add_message(
                        name,
                        ciphertext.clone(),
                        epk,
                        nonce,
                        send_log,
                    );
                    ui.session.save("session.json");

                    // Reset input field
                    ui.input_value.clear();
                }
            }
        }
        Message::SelectContact(name) => {
            ui.selected_contact = Some(name);
        }
    }
}

/*
 * The view function (Elm-style).
 *
 * It renders the interface based on the current `UI` state:
 *  - Left column  : list of contacts
 *  - Right column : chat history (decrypted) + input field
 *
 * For each stored message:
 *  - Plaintext is shown in white
 *  - Encryption/decryption logs are displayed in semi-transparent gray
 */
pub fn view(ui: &UI) -> Element<Message> {
    /* Build the left column (contact list) */
    let mut contacts_col: Column<Message> = column![].spacing(10);

    for u in &ui.contacts.users {
        let name = u.username();
        let selected = ui.selected_contact.as_deref() == Some(name);
        let label = if selected { format!("> {}", name) } else { name.to_string() };

        let contact_btn = button(text(label))
            .width(Length::Fill)
            .on_press(Message::SelectContact(name.to_string()))
            .style(|_theme: &Theme, _status| iced::widget::button::Style {
                background: Some(Background::Color(color!(0x1E1E2E))),
                text_color: Color::WHITE,
                border: Border {
                    radius: Radius::from(5.0),
                    ..Default::default()
                },
                ..Default::default()
            });

        contacts_col = contacts_col.push(contact_btn);
    }

    let contacts_list = container(contacts_col)
        .width(Length::Fixed(180.0))
        .height(Length::Fill)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(color!(0x181824))),
            text_color: Some(Color::WHITE),
            ..Default::default()
        });

    /* Build the right column (messages + input) */
    let mut messages_col = column![].spacing(8).padding(10);

    if let Some(name) = &ui.selected_contact {
        if let Some(messages) = ui.session.get_messages(name) {
            for stored in messages {
                if let (Some(epk_bytes), Some(nonce_bytes)) = (
                    box_::PublicKey::from_slice(&stored.ephemeral_pk),
                    box_::Nonce::from_slice(&stored.nonce),
                ) {
                    if let Some(user) = ui.contacts.get(name) {
                        if let Some((plaintext, recv_log)) = user.decrypt_message_with_logs(
                            &epk_bytes,
                            &nonce_bytes,
                            &stored.ciphertext,
                            ui.current_user.username(),
                        ) {
                            /* 1) Show decrypted plaintext message */
                            messages_col = messages_col.push(
                                text(format!(
                                    "{} → {}: {}",
                                    ui.current_user.username(),
                                    name,
                                    plaintext
                                ))
                                .color(Color::WHITE),
                            );

                            /* 2) Show sender log (encryption details) */
                            messages_col = messages_col.push(
                                text(&stored.log)
                                    .size(12)
                                    .color(Color {
                                        r: 0.7,
                                        g: 0.7,
                                        b: 0.7,
                                        a: 0.6,
                                    }),
                            );

                            /* 3) Show receiver log (decryption details) */
                            messages_col = messages_col.push(
                                text(recv_log)
                                    .size(12)
                                    .color(Color {
                                        r: 0.7,
                                        g: 0.7,
                                        b: 0.7,
                                        a: 0.6,
                                    }),
                            );
                        }
                    }
                }
            }
        }
    }

    let messages_scroll = scrollable(messages_col)
        .height(Length::FillPortion(8))
        .width(Length::Fill);

    /* Input row: message text field + send button */
    let input = text_input("Écrire un message...", &ui.input_value)
        .on_input(Message::InputChanged)
        .on_submit(Message::Send)
        .padding(10)
        .size(16)
        .width(Length::Fill);

    let send_button = button(text("Envoyer"))
        .on_press(Message::Send)
        .style(|_theme: &Theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(color!(0x1E1E2E))),
            text_color: Color::WHITE,
            border: Border {
                radius: Radius::from(5.0),
                ..Default::default()
            },
            ..Default::default()
        });

    let input_row: Row<Message> = row![input, send_button]
        .spacing(10)
        .align_y(Alignment::Center);

    let chat_col = column![messages_scroll, input_row].spacing(10).padding(10);

    /* Final layout: left = contacts, right = chat */
    row![contacts_list, chat_col].into()
}
