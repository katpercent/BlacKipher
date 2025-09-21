# BlacKipher 🔐

BlacKipher est une messagerie **end-to-end encryptée (E2EE)** écrite en **Rust**, inspirée du protocole de Signal.
Ce projet a pour but d’expérimenter les concepts de X3DH, Double Ratchet et la gestion des PreKeys.

---

## 📦 Installation

### 1. Prérequis

* [Rust](https://www.rust-lang.org/tools/install) (édition 2024)
* [libsodium](https://libsodium.gitbook.io/doc/installation) installé sur votre système :

#### Ubuntu/Debian :

```bash
sudo apt update
sudo apt install libsodium-dev
```

#### Arch Linux :

```bash
sudo pacman -S libsodium
```

#### macOS (Homebrew) :

```bash
brew install libsodium
```

---

### 2. Cloner le projet

```bash
git clone git@github.com:katpercent/BlacKipher.git
cd BlacKipher
```

---

### 3. Compiler

```bash
cargo build
```

---

### 4. Lancer

```bash
cargo run
```

---

## 📂 Structure du projet

```
BlacKipher/
├── Cargo.toml
└── src/
    ├── main.rs           # Point d'entrée
    └── client/
        ├── mod.rs
        ├── user.rs       # Struct User + génération des clés
        ├── contacts.rs
        └── sessions.rs
```

---

## 🖯 Roadmap

* [x] Génération des clés (identity, signed prekey, one-time prekeys)
* [ ] Implémentation du X3DH handshake
* [ ] Implémentation du Double Ratchet
* [ ] Serveur de distribution des PreKeys
* [ ] Envoi/réception de messages chiffrés
* [ ] Multi-device
* [ ] UI/CLI

---

## 📜 Licence

MIT © 2025 [katpercent](https://github.com/katpercent)

