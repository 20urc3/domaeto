# Domaeto 🌱🍅
Domaeto is a Rust-based fork of Domato, the minimalist DOM fuzzing generator. The name "Domaeto" is a lighthearted play on "tomato, tomaeto," because, well... it's pretty much the same thing, just rewritten in Rust! 🍅 
This project aims to bring the efficiency and safety of Rust to DOM fuzzing, offering a familiar experience for Domato enthusiasts while laying the groundwork for future improvements.

## 🛠️ Core Functionality

Domaeto preserves the essence of Domato by generating syntactically correct DOM scripts, designed to help uncover browser vulnerabilities.
🦀 Rust-Powered Advantages

    Memory safety with zero-cost abstractions.
    Faster execution, leveraging Rust's performance capabilities.
    Easier integration into modern Rust-based workflows.

🚀 Planned Enhancements

    Coverage-Guided Fuzzing: Add built-in support for coverage to maximize fuzzing efficiency and effectiveness.
    Additional customizability and optimizations for DOM fuzzing workflows.

## Why Domaeto?
Domato is a fantastic tool, but being in python limits its capabilities. By rewriting the project it Rust it becomes compatible with LibAFL which is very handy to integrate in a more complex fuzzer, or add features.

## Getting started
Clone the Repository:
```
git clone https://github.com/yourusername/domaeto.git  
cd domaeto
```
Build with Cargo:
`cargo build --release`  
Run:
`./target/release/domaeto`  
## Contributing
Contributions are welcome! Whether it's improving code quality, adding coverage support, or just fixing typos, your help makes Domaeto better for everyone.
Domaeto: "It’s basically the same thing, but Rustier!"
