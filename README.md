# SecureVault

![SecureVault Logo](https://via.placeholder.com/150/0077B6/FFFFFF?text=SecureVault)

**Secure Your Digital World with Confidence and Control**

[![Rust](https://img.shields.io/badge/Rust-1.60+-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/mafew-cyb/securevault)

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Security Features](#security-features)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

SecureVault is a **Rust-powered password management tool** that prioritizes security, reliability, and ease of use. Built with modern cryptographic standards, it provides a secure environment for managing your digital credentials.

### Why Choose SecureVault?

✅ **Zero-Trust Architecture**: Your data never leaves your device
✅ **Military-Grade Encryption**: AES-256-GCM for all stored data
✅ **Open Source**: Fully auditable codebase
✅ **Cross-Platform**: Works on Windows, macOS, and Linux
✅ **No Cloud Dependency**: All data stays local by default

---

## Features

### 🔐 Core Security
- **AES-256-GCM Encryption** for all stored credentials
- **Master Password Protection** with PBKDF2 key derivation
- **Secure Memory Handling** to prevent cold boot attacks
- **Clipboard Auto-Clear** after 30 seconds

### 🖥️ User Interface
- **Intuitive GUI** built with egui
- **Dark/Light Mode** support
- **Responsive Design** for all screen sizes
- **Keyboard Shortcuts** for power users

### 🔑 Password Management
- **Secure Password Generation** with customizable rules
- **Password Strength Analysis**
- **Category Tagging** for easy organization
- **Search & Filter** functionality
- **Bulk Import/Export** (JSON, CSV)

### 📜 Audit & History
- **Action Logging** with timestamps
- **Password Change Tracking**
- **Access History** for sensitive entries
- **Exportable Reports**

---

## Getting Started

### Prerequisites

- **Rust** (1.60 or higher)
- **Cargo** (Rust package manager)
- **Git** (for cloning the repository)

#### On Ubuntu/Debian:
```bash
sudo apt update
sudo apt install rustc cargo git
```
On macOS (using Homebrew):
```
brew install rust git
```

On Windows (using Winget):
```
winget install --id Rustlang.Rustup
winget install --id Git.Git
```
Installation

Clone the repository:
```
git clone https://github.com/mafew-cyb/securevault.git
cd securevault
```

Build the project:
```
cargo build --release
```

Usage
First Run

Launch SecureVault:
```
cargo run
```

Set up your master password (this will encrypt all your data)

Start adding your credentials securely
