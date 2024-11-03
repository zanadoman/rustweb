# 🌐 Rust CRUD Website

**A simple and efficient CRUD web application built with Rust, and Axum,
featuring a minimalist frontend using only HTML, HTMX, and Bootstrap.**

---

## ✨ Features & Goals

- **Simple and Minimalistic:** Lightweight web application without complex
  front-end framework or custom styling.
- **Rust-Powered Performance:** Built with Rust and Axum for efficient handling
  of concurrent requests and optimal server performance.
- **Interactive Without JavaScript:** Uses HTMX and Bootstrap to deliver a
  responsive, interactive experience without any custom JavaScript, making it
  ideal for pure HTML approach.
- **Quick Setup and Deployment:** Streamlined setup process with minimal
  dependencies, enabling easy deployment across different environments.

---

## 🛠️ Dependencies

To host the website, make sure you have the following depencencies installed:

- **Rustup**
- **MariaDB**
- **sqlx-cli**

---

## 🔧 Hosting Instructions

Follow these steps to host the website:
```
git clone https://github.com/zanadoman/rustweb.git
cd rustweb
cargo install sqlx-cli
mariadb -u root -p -e "CREATE DATABASE messages;"
cp .env.example .env
cargo sqlx migrate run
cargo run
```

---

***🚀 Enjoy!*** - Zana Domán
