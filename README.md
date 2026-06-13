# todo-rs
This is a simple command line based todo list tool built with Rust.
The purpose of this project is purely educational and to get my hands into some Rust based projects after completing the Rust book and the Rustlings exercises.

## Usage
You can either invoke the cli using cargo to build the binary each time (ideal for development) using `cargo run <command>` or access the binary file directly in `todo_rs/target/debug/todo_rs <command>` You can add the binary to your path if you want to use it anywhere in the system. 

Adding item to list:
```rust
todo_rs add milk
```

Listing all items:
```rust
todo_rs list
```
Which will print out:
```sh
0 - eggs: []
1 - milk: []
```

You can use the id number of items to mark them as done or remove them:
```rust
todo_rs done 0
```

```rust
todo_rs remove 0
```

## Persistence
The program expects to write to a `.todo_rs.json` file in your user's home directory. 
If it does not exist on first run it will be made for you.

## Work to be done:
- Filters
  - Show incomplete tasks only on list.
  - Bulk remove completed tasks.
