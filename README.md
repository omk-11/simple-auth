---
 🔐 Simple Rust Password Auth System

A lightweight password authentication system built in **Rust**, using **MD5 hashing** and a minimal HTML frontend.

---

 ✨ Features

- 📜 Password hashing using `md5` crate.
- 💡 Simple HTML + JS frontend with a modern yellow/black design.
- ⚡ Fast and lightweight — perfect for learning or small-scale use.


---

 🚀 Getting Started

 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

 1. Clone the Repository

```bash
git clone https://github.com/omk-11/simple_auth.git
cd simple_auth
```

 2. Run the Server

```bash
cargo run
```

> Your server will start locally. Open the browser and go to `http://localhost:8000` to access the form.

---

 🛠 How It Works

- User enters a password in the form.
- The password is sent to the backend.
- Rust backend hashes the password with **MD5**.
- The hashed value is matched against a stored hash to allow or deny access.

---

 🧪 Example

```rust
let stored_hash = "5f4dcc3b5aa765d61d8327deb882cf99"; // "password"
let user_input = md5::compute(password);
```

---

 ⚠️ Note

> **MD5 is not secure** for real-world authentication.  
This project is intended for **learning purposes only**. For secure applications, consider using libraries like `argon2`, `bcrypt`, or `scrypt`.

---

 📜 License

MIT

---
