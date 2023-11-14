# fan-control

# Steps

- [ ] finish hardware crate
    - [x] impl windows code
    - [x] impl linux code
    - [ ] test on real hardware
- [ ] package multiple format with [cargo-bundle](https://github.com/burtonageo/cargo-bundle)
    - [ ] Msi
    - [x] Deb
    - [ ] ARM support
    - [ ] RPM (not yet available)
    - [ ] EXE (not yet available)
    - [ ] Flatpak (not yet available)
    - [ ] Snap (not yet available)
    - [ ] CI for packaging
- [ ] change iced to libcosmic (this will enable new widgets, like dropdown) (libcosmic must be ported to Windows before)
    - [ ] theme
    - [ ] impl UI for managing configs
    - [ ] impl UI for removing/adding nodes
    - [ ] impl UI settings page
- [ ] impl UI graph behavior
- [ ] icons
- [ ] tray icon support (not yet available on [Iced](https://whimsical.com/roadmap-iced-7vhq6R35Lp3TmYH4WeYwLM))
- [ ] i18n support ([example](https://github.com/pop-os/cosmic-edit/blob/master_jammy/Cargo.toml))
    - [x] init file structure
    - [ ] add all string to ftl files

## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md) and [hardware](./hardware/README.md)
- the app: integrate all this crates in one executable


# Build
See instructions [here](./BUILD.md).

# Contributing
Contributions are welcome, do not hesitate to open an issue, a pull request, etc...