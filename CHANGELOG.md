# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-06

### Added

- GTK4 graphical interface with 4 tabs: Information, Peripherals, Settings, Interface
- Real-time battery monitoring with 5-second auto-refresh
- Charge threshold management (start/stop) via sysfs
- Discharge alarm configuration
- Systemd service for persistent threshold restoration at boot
- Multi-vendor support: ASUS, Lenovo/ThinkPad, Dell, Huawei, System76, Tuxedo, Samsung, Sony, LG, MSI, Toshiba, MacBook
- Peripheral battery monitoring (HID++ devices: mouse, keyboard)
- Internationalization (French / English)
- Dark / Light theme with live switching
- Debug mode (`--debug`) with structured colored logging
- `.deb` package generation script
- Install / Uninstall scripts
- Offline HTML documentation (FR/EN)
- Path traversal protection on battery name input
- Privilege escalation via `pkexec` only when applying settings
- CI/CD via GitHub Actions (build, test, clippy, fmt)

### Security

- Input validation on all sysfs paths to prevent path traversal
- Numeric validation before shell script construction
- pkexec availability check before privilege escalation
