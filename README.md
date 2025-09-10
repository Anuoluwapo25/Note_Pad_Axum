# 📘 README for Local PostgreSQL Note Pad Application

## 🌟 Overview
This project is a **Rust-based Note Pad API** built with the **Axum web framework** and **PostgreSQL**.  
It provides a **RESTful API** for performing CRUD (Create, Read, Update, Delete) operations on notes, allowing you to manage notes with titles and content efficiently.  
The application is designed to run locally, making it ideal for **development and learning purposes**.

---

## ✨ Features
- 📌 **RESTful API**: Full CRUD operations for managing notes.  
- 🐘 **PostgreSQL Integration**: Uses SQLx for async database operations.  
- ⚡ **Axum Framework**: Leverages Rust's async capabilities for high performance.  
- 🚦 **Structured Error Handling**: Comprehensive error management with proper HTTP status codes.  
- 💻 **Local Development Focus**: Designed to run with a local PostgreSQL instance.  

---

## 🛠️ Technologies Used
- 🦀 Rust  
- 🚀 Axum  
- 🔗 SQLx  
- ⏱️ Tokio  
- 🐘 PostgreSQL  
- 📅 Chrono  
- 🆔 UUID  

---

## 📦 Installation & Setup

### 🔑 Prerequisites
- **Rust Toolchain** → [Install Rust](https://www.rust-lang.org/tools/install)  
- **PostgreSQL** → On macOS, install via Homebrew:
  ```bash
  brew install postgresql
  brew services start postgresql
