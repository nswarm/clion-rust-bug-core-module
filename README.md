## Environment

* **IntelliJ Rust plugin version:** 0.4.160.4261-212
* **Rust toolchain version:** 1.56.1 (stable-x86_64-pc-windows-msvc)
* **IDE name and version:** CLion 2021.2.3
* **Operating system:** Windows 10 (Version 10.0.19042 Build 19042)

## Problem description

When a user module is named `core`, some code inspections & autocompletions stop working.

Examples I encountered:
- Certain #[derive(TRAIT)] traits were no longer recognized such as Debug and Default.
- Using the `?` operator on Results would be unable to deduce the type.

## Steps to reproduce

1. Create a new project
2. asd

See project files:
- Top-level Cargo.toml - where `core` is defined as a workspace member.
- example/src/main.rs - some examples of failing type deductions.

## Workaround

Don't create a user module named `core`.

Maybe an obvious danger in hindsight, but in my case I had created the folder and linked it in the workspace without thinking about it shadowing rust's core lib, and spent a solid hour trying to figure out why my IDE was only half working.

