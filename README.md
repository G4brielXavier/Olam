# Olam

![Crates.io Version](https://img.shields.io/crates/v/olam?style=flat-square&color=orange)
![License](https://img.shields.io/badge/license-BSD3-blue?style=flat-square)
![Rust](https://img.shields.io/badge/rust-v1.70%2B-black?style=flat-square&logo=rust)

*High-performance CLI for worldbuilders and storytellers to manage complex timelines and chronologies.* Built in **Rust** with high-speed memory-mapping and powered by the [Tequel](https://github.com/G4brielXavier/TEQUEL) cryptographic engine.

*by Gabriel "dotxav" Xavier*

![OlamBanner](./img/olam_banner.png)

---

## 📖 Summary

- [Olam](#olam)
  - [📖 Summary](#-summary)
  - [What's Olam?](#whats-olam)
  - [✨ Features](#-features)
  - [📃 Documentation](#-documentation)
  - [🏃 Quick Start](#-quick-start)
    - [**1.** Evoke your Space and Immerse](#1-evoke-your-space-and-immerse)
    - [2. Create a Timeline and Immerse](#2-create-a-timeline-and-immerse)
    - [3. Anchor a Year](#3-anchor-a-year)
    - [4. Define an Age](#4-define-an-age)
    - [5. Add Events](#5-add-events)
    - [6. Preview your Creation](#6-preview-your-creation)
  - [📥 Installation](#-installation)
    - [Cargo (Cross-platform)](#cargo-cross-platform)
    - [Scoop (Windows)](#scoop-windows)
  - [⚖️ License](#️-license)

---

## What's Olam?

Olam was born from a storyteller's need for a professional tool to organize vast fictional universes. Unlike general-purpose note-taking apps, Olam is a dedicated **Chronology Engine**. It allows you to "immerse" yourself in specific spaces and timelines, managing eras, ages, and events with surgical precision and sovereign data security.

## ✨ Features

- 🔐 **Secure by Default:** All your data is encrypted locally using the **Tequel** engine.
- ⚡ **Built for Speed:** Uses `mmap` (memory-mapped files) and SIMD-optimized logic for near-instant execution.
- ⚓ **Contextual Navigation:** Immerse into your spaces and timelines with `immerse` and `emerge` commands.
- 🛠️ **Fully Customizable:** Create your own calendar units (Years, Eras, etc.).
- 📦 **Zero Cloud:** 100% offline. Your space, your data, your privacy.

## 📃 Documentation

- **Full Reference:** [Olam Documentation on Docs.rs]()
- [⚓ Navigation Guide](./docs/Navigation.md)
- [🌌 Managing Spaces](./docs/Spaces.md)
- [⏳ Timeline Logic](./docs/Timelines.md)

---

## 🏃 Quick Start

### **1.** Evoke your Space and Immerse
```bash
olam evoke "My Space" -i
```

### 2. Create a Timeline and Immerse
```bash
olam add "Main Timeline" -i
```

### 3. Anchor a Year
```bash
olam year 0 "Genesis"
```

### 4. Define an Age
```bash
olam age "First Age" -d 100
```

### 5. Add Events
```bash
# Add a note in < Between >
olam age "Fire Age" -b "Discovery of roasted meat"

# Add a year in < Events >
olam year -a "Fire Age" 45 "The Great Volcano Eruption"
```

### 6. Preview your Creation
```bash
olam tl
```

**Output Preview**

```plaintext
(olam) ~MySpace.MainTimeline
——?—— 100 Year(s)
——?—— 1 Age(s)
——?—— Max 1000 Year(s)

-< Year 0 >-
    
    Genesis

-< Fire Age - Year 1 >-
    
    < Between >
        
        • Discovery of roasted meat

    < Events >
        
        -< Year 45 >- 
            
            • The Great Volcano Eruption

-< Fire Age Dissolved - Year 100 >-
```

## 📥 Installation

### Cargo (Cross-platform)

```bash
cargo install olam
```

### Scoop (Windows)

If you don't have **Scoop** installed yet, run this in your PowerShell:
```bash
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex
```
Then, add the **dotxav** bucket and install **Olam**:

```bash
scoop bucket add dotxav https://github.com/G4brielXavier/scoop-bucket
scoop install olam
```

## ⚖️ License

Olam is a gift to the worldbuilding and storytelling community, licensed under the **BSD 3-Clause License**. See the [LICENSE](./LICENSE) file for details.