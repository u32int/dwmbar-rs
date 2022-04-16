# dwmbar

A simple, clean and very modular status bar setter for the dwm window manager written in rust.

### Features

- Configured in the language it is written in

- Full hex color support*

- Easily extensible

*Note: requires [the status2d dwm patch](https://dwm.suckless.org/patches/status2d/)*

### Installation

The install.sh script copies the binary into /usr/local/bin. Simply run the binary as a process in your xinitrc.

### Example configurations

The default configuration

```rust
let modules = [
    "dwmbar".to_string(),
    mem::used(),
    clock::formatted("%Y-%m-%d %H:%M"),
];
// Define your separator here (it will be inserted between modules, optional)
let separator = " ";
```

<img alt="default-config" src="https://raw.githubusercontent.com/vshatgit/dwmbar/main/imgs/exampledefault.png">

A more complete configuration would look something like this.

```rust
let modules = [
        colors::foreground("#787878"),
        "WTR".to_string(),
        colors::foreground("#d8d8d8"),
        persistent.wttr.clone(),
        colors::foreground("#787878"),
        "UPD".to_string(),
        colors::foreground("#d8d8d8"),
        persistent.updates.clone(),
        colors::foreground("#787878"),
        "MEM ".to_string(),
        colors::foreground("#d8d8d8"),
        mem::used(),
        colors::foreground("#787878"),
        "CPU ".to_string(),
        colors::foreground("#d8d8d8"),
        cpu::average_load(),
        colors::background("#d8d8d8"),
        colors::foreground("#000000"),
        clock::formatted("%m-%d %H:%M"),
    ];
    // Define your separator here (it will be inserted between modules, optional)
    let separator = " ";
```

and would result in a bar looking like this

<img alt="default-config" src="https://raw.githubusercontent.com/vshatgit/dwmbar/main/imgs/exampleone.png">

#### Planned features

- [ ] Different refresh times for different modules (currently only implemented for persistent data)

- [ ] Many more modules



### Contributing

If you write a useful module or extend the programs existing functionality, feel free to send a pull request.
