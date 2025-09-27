# BlacKipher

**BlacKipher** is an **end-to-end encrypted (E2EE) messenger** written in **Rust**, inspired by the Signal protocol.  
It is an **educational and experimental project** exploring the concepts behind **X3DH**, **Double Ratchet**, and the management of **PreKeys**.

**Disclaimer**

BlacKipher is an **educational and experimental project**.  
It is designed to explore how modern end-to-end encryption protocols (like Signal’s X3DH and Double Ratchet) work, and to provide detailed logs for learning purposes.  

It **must not** be used for real-world secure communications, as:
- The protocol is incomplete and simplified,
- Security properties such as forward secrecy and post-compromise security are not fully guaranteed,
- Key distribution and authentication are not production-ready.

If you need actual secure messaging, for now, use battle-tested implementations such as [Signal](https://signal.org/) or [Matrix](https://matrix.org/).

---

## Installation

### 1. Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (2024 edition recommended)
* [libsodium](https://libsodium.gitbook.io/doc/installation) installed on your system:

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install libsodium-dev
````

#### Arch Linux

```bash
sudo pacman -S libsodium
```

#### macOS (Homebrew)

```bash
brew install libsodium
```

---

### 2. Clone the project

```bash
git clone git@github.com:katpercent/BlacKipher.git
cd BlacKipher
```

---

### 3. Build

```bash
cargo build
```

---

### 4. Run

```bash
cargo run
```

---

## Project structure

```
BlacKipher/
├── Cargo.toml
└── src/
    ├── main.rs           # Application entry point
    └── client/
        ├── mod.rs
        ├── user.rs       # User struct + key generation and crypto logic
        ├── contacts.rs   # Contact list management
        └── sessions.rs   # Persistent message sessions
    └── ui/
        ├── mod.rs
        └── app.rs        # Iced GUI (Elm-style architecture)
```

---

## Demo with Log Breakdown

![BlacKipher demo](screenshot/demo.bmp)

When **katpercent** sends the message `hey` to **alice**, the following happens under the hood:

### Sender (katpercent → alice)

```
== log ==
Sender: katpercent
Receiver: alice
Verify(peer.SPK signed by peer.ID) = true
Ephemeral PK: eda2220a8baba021f734d864e8bab172395d086ef1f278bc7bf9edb48e9a850d
DH(ephemeral, peer.SPK): precomputed (32 bytes)
Nonce: f8b897f98c25d3fd1e4f75835c13fbbd6611a5c451d53ec1
Ciphertext: 85ecc46e3245ba98d62b2f95015fb0c82e93d5
```

**Explanation:**

* The sender (`katpercent`) verifies that Alice’s **Signed PreKey (SPK)** is valid.
* A fresh **ephemeral public key** is generated.
* Using **Diffie-Hellman (DH)**, the sender derives a **32-byte shared secret** from the ephemeral key and Alice’s SPK.
* A **nonce** is generated to ensure uniqueness of the encryption.
* The plaintext `"hey"` is encrypted into the given **ciphertext**.

---

### Receiver (alice ← katpercent)

```
== log (recv) ==
Receiver: alice
Sender: katpercent
DH(sender.ephemeral, self.SPK): precomputed (32 bytes)
Nonce: f8b897f98c25d3fd1e4f75835c13fbbd6611a5c451d53ec1
Ciphertext: 85ecc46e3245ba98d62b2f95015fb0c82e93d5
Plaintext: hey
```

**Explanation:**

* Alice receives the ciphertext, along with the sender’s **ephemeral key** and the **nonce**.
* She recomputes the **DH shared secret** using her own SPK and the ephemeral key.
* With the shared secret + nonce, she can **decrypt** the ciphertext.
* The plaintext is successfully recovered: `"hey"`.

---

This illustrates how **BlacKipher** uses **ephemeral Diffie-Hellman exchanges** and **PreKeys** (inspired by X3DH) to encrypt messages end-to-end.


---

## Roadmap

**Educational prototype (current):**
- [x] Identity, Signed PreKeys, One-Time PreKeys generation  
- [x] Local message encryption/decryption with detailed logs (for learning)  
- [x] Basic GUI with Iced (contact list + chat view)  

**If targeting production:**
- [ ] Implement full X3DH handshake  
- [ ] Implement Double Ratchet key evolution  
- [ ] PreKey server for initial key distribution  
- [ ] Secure message exchange between devices  
- [ ] Multi-device support  
- [ ] CLI mode  

---

## Contributing

Contributions are welcome for **educational improvements** — bug fixes, better documentation, or clearer examples.  
Please open an issue or a pull request if you’d like to help.

---

## License

MIT © 2025 [katpercent](https://github.com/katpercent)
