# Dark Mode Toggle Applet for the COSMIC™ desktop

Toggle system light/dark mode from the panel.

## Installation

### Build and install

```sh
just build-release
sudo just install
```

### Add to the panel

After installing, the applet must be added to the panel manually:

1. Right-click the panel → **Panel Settings**
2. Go to **Applets**
3. Find **Dark Toggle** and add it to the panel

Log out and back in if the applet does not appear after adding it.

## Building from source

```sh
just build-debug    # debug build
just build-release  # release build
just install        # install to /usr/share and /usr/bin
```

Run with logging:

```sh
RUST_LOG="warn,cosmic_ext_applet_dark_toggle=debug" ./target/debug/cosmic-ext-applet-dark-toggle
```

## Credits

Originally created by [@maciekk64](https://github.com/maciekk64)
