# BlacKipher 🔐

**BlacKipher** is an **end-to-end encrypted (E2EE) messenger** written in **Rust**, inspired by the Signal protocol.  
It is an **educational and experimental project** exploring the concepts behind **X3DH**, **Double Ratchet**, and the management of **PreKeys**.

⚠️ **Disclaimer**

BlacKipher is an **educational and experimental project**.  
It is designed to explore how modern end-to-end encryption protocols (like Signal’s X3DH and Double Ratchet) work, and to provide detailed logs for learning purposes.  

It **must not** be used for real-world secure communications, as:
- The protocol is incomplete and simplified,
- Security properties such as forward secrecy and post-compromise security are not fully guaranteed,
- Key distribution and authentication are not production-ready.

If you need actual secure messaging, for now, use battle-tested implementations such as [Signal](https://signal.org/) or [Matrix](https://matrix.org/).

---

## 📦 Installation

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

## 📂 Project structure

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

## 🛠 Roadmap

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

## 📜 License

MIT © 2025 [katpercent](https://github.com/katpercent)
