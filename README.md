# rbmenu-tui

`rbmenu-tui` is a Terminal User Interface for [rbmenu](https://github.com/DevHyperCoder/rbmenu) 
written in Rust and [Cursive](https://github.com/gyscos/cursive/).

## Features
- Vim like keybindings
- Basic CRUD operations
- Filter with regex

## Installation
`rbmenu-tui` is available on [crates.io](https://crates.io/crates/rbmenu-tui)

**Arch Linux** : Available on AUR, `rbmenu-tui` for manual compilation from release and `rbmenu-tui-bin` for precompiled binary

**Manual Installation**
- Install the rust toolchain. `cargo` should be on the `$PATH`
- Clone the repo: `git clone https://github.com/DevHyperCoder/rbmenu-tui.git`. Change directory (`cd`) into the `rbmenu-tui` folder
- Build the code: `cargo build --release`
- Copy the binary to a location on $PATH. Binary is in `./target/release/rbmenu-tui`
- For operation with cargo, `cargo run -- <options>`.

## Keybindings

| Key      | Description                          |
|----------|--------------------------------------|
| ?        | Help                                 |
| a        | <url> <optional name> Add a new task |
| d        | :<id> OR <name> Remove a task        |
| x        | Remove selected                      |
| y        | Copy selected                        |
| l        | View selected                        |
| <Enter>  | View selected                        |
| j        | Move down                            |
| k        | Move up                              |
| e        | Edit bookmark                        |

## Screenshots

![`rbmenu-tui` home](./res/rbmenu-tui-home.png)
![`rbmenu-tui` help](./res/rbmenu-tui-help.png)
![`rbmenu-tui` add](./res/rbmenu-tui-add.png)
![`rbmenu-tui` edit](./res/rbmenu-tui-edit.png)

## Theme

Create a `$HOME/.local/share/rbmenu/theme.toml` file. If the file is present,
`rbmenu-tui` will use it, otherwise defaults to `cursive` default theme.

## License

rbmenu-tui is licensed under the GPL-3 license. Our copy can be found [here](./LICENSE).
