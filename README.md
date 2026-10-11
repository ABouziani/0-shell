# 0-shell

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Go Version](https://img.shields.io/badge/Go-%3E%3D1.20-00ADD8?style=flat&logo=go)](https://golang.org/)
[![Status](https://img.shields.io/badge/status-active-success.svg)]()

A lightweight, high-performance, and custom shell implementation designed for simplicity, efficiency, and extensibility.

</div>

---

## 📌 Table of Contents
- [About](#-about)
- [Features](#-features)
- [Getting Started](#-getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#-usage)
- [Architecture & Design](#-architecture--design)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🚀 About

**0-shell** is a custom Unix shell built from scratch to explore system programming, process management, input/output redirection, and low-level command execution. Designed with performance and clean code architecture in mind, it provides a robust REPL (Read-Eval-Print Loop) environment capable of handling standard shell operations smoothly.

---

## ✨ Features

- **Interactive REPL:** Clean prompt interface for command entry and execution.
- **Built-in Commands:** Native support for essential shell operations (e.g., `cd`, `exit`, `history`).
- **External Command Execution:** Forks child processes to execute system binaries seamlessly.
- **I/O Redirection:** Handles standard input/output redirection (`<`, `>`, `>>`) effortlessly.
- **Pipeline Support:** Connects multiple commands using pipes (`|`) for advanced command chaining.
- **Signal Handling:** Gracefully handles signals like `SIGINT` (Ctrl+C) and `SIGTSTP` (Ctrl+Z) without terminating the shell session.

---

## 📦 Getting Started

### Prerequisites

Make sure you have the following installed on your machine:
- **Go** (v1.20 or higher recommended)
- A Unix-like operating system (Linux / macOS)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ABouziani/0-shell.git
   cd 0-shell
   ```

2. **Build the binary:**
   ```bash
   go build -o 0-shell main.go
   ```

3. **Run the shell:**
   ```bash
   ./0-shell
   ```

---

## 💡 Usage

Once inside the shell, you can execute regular system commands, manage directories, or chain operations:

```bash
# Run basic commands
$ ls -la

# Change directories
$ cd /path/to/directory

# Use pipes
$ cat file.txt | grep "keyword"

# Redirect output
$ echo "Hello, World!" > output.txt

# Exit the shell
$ exit
```

---

## 🏗️ Architecture & Design

`0-shell` follows a modular software design pattern separating responsibilities into clear layers:
1. **Parser:** Tokenizes raw user input, stripping whitespace, handling quotes, and identifying operator tokens (`|`, `>`, `<`).
2. **Evaluator / Executor:** Determines whether a command is a shell builtin or an external system binary, managing process lifecycles, file descriptors, and pipes.
3. **Process Manager:** Manages background/foreground job states and signal propagation.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! 
Feel free to check the [issues page](https://github.com/ABouziani/0-shell/issues).

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git origin push feature/AmazingFeature`)
5. Open a Pull Request

---

## 📜 License

Distributed under the **MIT License**. See `LICENSE` for more information.