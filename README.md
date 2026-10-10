# Rust by Example: a folder-by-folder guide

A beginner-friendly Cargo workspace of small programs covering the core ideas in
[The Rust Programming Language](https://doc.rust-lang.org/book/) and the
[Rust standard library](https://doc.rust-lang.org/std/).

Each folder under `chapters/` is an independent binary crate. Read its `src/main.rs`,
then run it from the repository root:

```sh
cargo run -p chapter-01-getting-started
cargo test --workspace
```

Some examples read from standard input or a file; their source explains the expected
input. The workspace uses only the standard library.

## Chapter folders

| Folder | Topic | Try |
| --- | --- | --- |
| `01-getting-started` | `main`, printing, and Cargo | `cargo run -p chapter-01-getting-started` |
| `02-guessing-game` | Input, loops, parsing, and comparisons | `cargo run -p chapter-02-guessing-game` |
| `03-common-concepts` | Variables, types, functions, and control flow | `cargo run -p chapter-03-common-concepts` |
| `04-ownership` | Ownership, borrowing, and slices | `cargo run -p chapter-04-ownership` |
| `05-structs` | Structs, methods, and derived traits | `cargo run -p chapter-05-structs` |
| `06-enums` | Enums, `Option`, and `match` | `cargo run -p chapter-06-enums` |
| `07-modules` | Modules, visibility, and paths | `cargo run -p chapter-07-modules` |
| `08-collections` | Vectors, strings, and hash maps | `cargo run -p chapter-08-collections` |
| `09-error-handling` | `Result`, `?`, and recoverable errors | `cargo run -p chapter-09-error-handling` |
| `10-generics-traits-lifetimes` | Generic functions, traits, and lifetimes | `cargo run -p chapter-10-generics-traits-lifetimes` |
| `11-testing` | Unit tests and assertions | `cargo test -p chapter-11-testing` |
| `12-closures-iterators` | Closures and iterator adapters | `cargo run -p chapter-12-closures-iterators` |
| `13-smart-pointers` | `Box`, `Rc`, and `RefCell` | `cargo run -p chapter-13-smart-pointers` |
| `14-concurrency` | Threads, message passing, and shared state | `cargo run -p chapter-14-concurrency` |
| `15-patterns` | Destructuring and `if let` | `cargo run -p chapter-15-patterns` |
| `16-advanced-rust` | Newtypes, type aliases, and trait objects | `cargo run -p chapter-16-advanced-rust` |
| `17-cli-project` | A small file-search command-line program | `cargo run -p chapter-17-cli-project -- needle file.txt` |

For async programming, see the
[Rust Book's async chapter](https://doc.rust-lang.org/book/ch17-00-async-await.html);
async code needs an executor to poll futures, so it is not presented as a
standard-library-only runnable example here.
