SECUREVAULT

Secure Your Digital World with Confidence and Control

last-commit repo-top-language repo-language-count
Built with the tools and technologies:

Markdown Rust TOML

Table of Contents

Overview
Getting Started
Prerequisites
Installation
Usage
Testing
Overview

SecureVault is a Rust-powered password management tool focused on safeguarding sensitive data through robust encryption and secure workflows. It offers an intuitive GUI for managing, generating, and copying passwords, all while maintaining high security standards.

Why SecureVault?

This project aims to provide a secure, reliable, and user-friendly solution for credential management. The core features include:

🛡️ Encryption & Secure Storage: Utilizes AES-GCM encryption to protect your passwords and sensitive data.
🔑 Multi-Factor Authentication: Supports password hashing and TOTP for enhanced security.
🎨 Intuitive User Interface: Enables seamless password management, generation, and clipboard copying.
📜 Audit & History Logs: Maintains detailed records of user actions for troubleshooting and compliance.
⚙️ Customizable Password Generator: Creates strong, tailored passwords to meet diverse security needs.
Getting Started

Prerequisites

This project requires the following dependencies:

Programming Language: Rust
Package Manager: Cargo
Installation

Build securevault from the source and install dependencies:

Clone the repository:

❯ git clone https://github.com/mafew-cyb/securevault
Navigate to the project directory:

❯ cd securevault
Install the dependencies:

Using cargo:

❯ cargo build
Usage

Run the project with:

Using cargo:

cargo run
Testing

Securevault uses the {test_framework} test framework. Run the test suite with:

Using cargo:

cargo test
⬆ Return
