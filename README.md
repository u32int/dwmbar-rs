# dwmbar

A simple, clean and very modular status bar setter for the dwm window manager written in rust.



### Features

- Configured in the language it is written in

- Full hex color support*

- Easily extensible

*Note: requires [the status2d dwm patch](https://dwm.suckless.org/patches/status2d/)*



### Example configurations

The default configuration

```rust
let modules = [
    "dwmbar".to_string(),
    modules::mem::used(),
    modules::clock::formatted("%Y-%m-%d %H:%M"),
];
// Define your separator here (it will be inserted between modules, optional)
let separator = " ";
```

default-screenshot-here



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
        modules::mem::used(),
        colors::foreground("#787878"),
        "CPU ".to_string(),
        colors::foreground("#d8d8d8"),
        modules::cpu::average_load(),
        colors::background("#d8d8d8"),
        colors::foreground("#000000"),
        modules::clock::formatted("%m-%d %H:%M"),
    ];
    // Define your separator here (it will be inserted between modules, optional)
    let separator = " ";


```

and would result in a bar looking like this

second-screenshot-here


### Contributing

If you write a useful module or extend the programs existing functionality, feel free to send a pull request.
