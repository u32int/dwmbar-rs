# dwmbar

A modular status bar setter for dwm, written in rust.

### Features

- Configured in the language it is written in

- Full hex color support*

- Custom refresh rates per module (or even module instance!)

- Easily extensible

*Note: requires [the status2d dwm patch](https://dwm.suckless.org/patches/status2d/)*

### Installation

The install.sh script copies the binary into /usr/local/bin. Simply run the binary as a process in your xinitrc/init script.

### Example configurations

The configuration can be found in the main.rs in the main function. This is the default configuration.

```rust
let modules: Vec<&dyn BarModule> = vec!{
    &Text { text: "dwmbar" },
    &Mem {
        format: "{used}",
        refresh_rate: 5,
        unit: MemoryUnit::MB,
    },
    &Clock {
        format: "%m-%d %H:%M",
        refresh_rate: 1,
    },
};

let separator = " ";
```

<img alt="default-config" src="https://raw.githubusercontent.com/vshatgit/dwmbar/main/imgs/exampledefault.png">

A more complete configuration would look something like this.

```rust
let modules: Vec<&dyn BarModule> = vec!{
    &Color { background: None, foreground: Some("#787878"), },
    &Text { text: "WTR" },
    &Color { background: None, foreground: Some("#d8d8d8"), },
    &Wttr {
        location: "Warsaw",
        refresh_rate: 3600,
    },
    &Color { background: None, foreground: Some("#787878"), },
    &Text { text: "UPD" },
    &Color { background: None, foreground: Some("#d8d8d8"), },
    &Updates {
        format: "{count}",
        refresh_rate: 7200,
        update_cmd: "checkupdates",
    },
    &Color { background: None, foreground: Some("#787878"), },
    &Text { text: "MEM" },
    &Color { background: None, foreground: Some("#d8d8d8"), },
    &Mem {
        format: "{used}",
        refresh_rate: 5,
        unit: MemoryUnit::MB,
    },
    &Color { background: None, foreground: Some("#787878"), },
    &Text { text: "CPU" },
    &Color { background: None, foreground: Some("#d8d8d8"), },
    &Cpu {
        format: "{load}",
        refresh_rate: 1,
    },
    &Color { background: Some("#d8d8d8"), foreground: Some("#000000"), },
    &Clock {
        format: "%m-%d %H:%M",
        refresh_rate: 1,
    },
};

let separator = " ";


```

and would result in a bar looking like this

<img alt="default-config" src="https://raw.githubusercontent.com/vshatgit/dwmbar/main/imgs/exampleone.png">

#### Planned features

- [x] Different refresh times for different modules (currently only implemented for persistent data)

- [ ] Many more modules

### Contributing

If you write a useful module or extend the programs existing functionality, feel free to send a pull request.
