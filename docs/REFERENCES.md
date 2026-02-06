# References / Références

Ce fichier est une _cheat-sheet_ structurée pour Rust/GTK4 + outils système utilisés dans ce projet.

Il contient 2 versions :

- Français : bloc `BEGIN:FR`
- English : bloc `BEGIN:EN`

---

<!-- BEGIN:FR -->

# Références Rust & GTK4 (FR)

## 1) Documentation officielle

- Rust (FR) : https://www.rust-lang.org/fr/
- Rust Book (EN) : https://doc.rust-lang.org/book/
- Cargo Book : https://doc.rust-lang.org/cargo/
- Rust API docs : https://doc.rust-lang.org/std/
- gtk-rs book (GTK4) : https://gtk-rs.org/gtk4-rs/stable/latest/book/
- GTK Docs : https://docs.gtk.org/

## 2) Commandes Cargo (quotidien)

### Nettoyage

```bash
cargo clean                 # Supprime target/
```

### Build / Run

```bash
cargo check                 # Vérification rapide (sans binaire)
cargo build                 # Build debug
cargo build --release       # Build optimisé

cargo run                   # Build + run debug
cargo run --release         # Build + run release
cargo run -- --debug        # Passe des args au binaire
```

### Tests

```bash
cargo test                  # Lance tous les tests
cargo test -- --nocapture   # Affiche println!
cargo test -- --test-threads=1  # Exécution séquentielle
```

### Qualité

```bash
cargo fmt                   # Formatage
cargo clippy                # Linter
cargo clippy --fix          # Auto-fix (prudence)
```

### Documentation

```bash
cargo doc                   # Génère la doc
cargo doc --open            # Génère + ouvre dans le navigateur
```

### Dépendances

```bash
cargo add <crate>           # Ajoute une dépendance
cargo update                # Met à jour les dépendances
cargo tree                  # Arbre des deps
```

## 3) Clippy : niveaux “AAA”

### Basique

```bash
cargo clippy
```

### Strict (pedantic)

```bash
cargo clippy -- -W clippy::pedantic
```

### Expérimental (nursery)

```bash
cargo clippy -- -W clippy::nursery
```

### Refuser les warnings

```bash
cargo clippy -- -D warnings
```

### Check AAA (recommandé)

```bash
cargo clippy --all-features --all-targets -- \
  -W clippy::pedantic \
  -W clippy::nursery
```

## 4) Fixes Clippy courants

### Littéraux illisibles

```rust
// ❌ Avant
let x = 4000000;

// ✅ Après
let x = 4_000_000;
```

### Clones inutiles

```rust
// ❌ Avant
let s = my_string.clone();

// ✅ Après (si possible)
let s = &my_string;
```

### Docs manquantes

```rust
// ❌ Avant
pub fn calculate(x: i32) -> i32 { x * 2 }

// ✅ Après
/// Multiplie l'entrée par 2
pub fn calculate(x: i32) -> i32 { x * 2 }
```

## 5) Développement GTK4 (Linux)

### Dépendances Debian/Ubuntu

```bash
sudo apt update
sudo apt install libgtk-4-dev build-essential policykit-1
```

### Debug GTK

```bash
G_MESSAGES_DEBUG=all battery-manager --debug
```

### Logs Battery Manager : couleurs (optionnel)

Par défaut, la coloration suit le terminal (auto).

```bash
# Forcer les couleurs (même si stdout n'est pas un TTY)
BATTERY_MANAGER_COLOR=always battery-manager --debug

# Désactiver toutes les couleurs (standard)
NO_COLOR=1 battery-manager --debug
```

## 6) Outils système utiles (projet)

### Arborescence

```bash
tree -L 2 -I 'target' --dirsfirst
```

### Compter les lignes de code

```bash
cloc src/ --quiet
```

### Recherche rapide (ripgrep)

```bash
rg "pattern" -n src/
```

## 7) systemd : service de restauration

```bash
systemctl status battery-manager.service
systemctl restart battery-manager.service
journalctl -u battery-manager.service -b --no-pager
```

## 8) pkexec / PolicyKit

Le projet utilise `pkexec` pour obtenir les droits root au moment d'écrire dans `/sys/class/power_supply/`.

À tester :

```bash
pkexec --version
```

## 9) sysfs : vérification compatibilité seuils

```bash
ls /sys/class/power_supply/BAT0/
cat /sys/class/power_supply/BAT0/status

# Exemples (selon fabricant)
cat /sys/class/power_supply/BAT0/charge_control_end_threshold 2>/dev/null || true
cat /sys/class/power_supply/BAT0/charge_stop_threshold 2>/dev/null || true
```

## 10) Packaging (.deb)

```bash
cd install
./build-deb.sh
```

## 11) Workflow essentiel

```bash
# Cycle dev
cargo check
cargo run

# Avant PR / release
cargo fmt
cargo clippy --all-features --all-targets -- -W clippy::pedantic -W clippy::nursery
cargo test
cargo build --release
```

## 12) Exemple Cargo.toml

```toml
[package]
name = "battery_manager"
version = "1.0.0"
edition = "2021"

[dependencies]
gtk4 = "0.10"
glib = "0.21"
dirs = "6.0"
libc = "0.2"

[profile.release]
opt-level = 3
lto = true
```

<!-- END:FR -->

---

<!-- BEGIN:EN -->

# Rust & GTK4 References (EN)

This file is a structured cheat-sheet for Rust/GTK4 + the system tools used by this project.

## 1) Official documentation

- Rust (FR landing) : https://www.rust-lang.org/fr/
- The Rust Book : https://doc.rust-lang.org/book/
- The Cargo Book : https://doc.rust-lang.org/cargo/
- Rust API docs : https://doc.rust-lang.org/std/
- gtk-rs book (GTK4) : https://gtk-rs.org/gtk4-rs/stable/latest/book/
- GTK Docs : https://docs.gtk.org/

## 2) Cargo commands (daily)

### Cleanup

```bash
cargo clean                 # Removes target/
```

### Build / Run

```bash
cargo check                 # Fast type-check (no binary)
cargo build                 # Debug build
cargo build --release       # Optimized build

cargo run                   # Build + run debug
cargo run --release         # Build + run release
cargo run -- --debug        # Pass args to the binary
```

### Testing

```bash
cargo test                  # Run all tests
cargo test -- --nocapture   # Show println!
cargo test -- --test-threads=1  # Run sequentially
```

### Quality

```bash
cargo fmt                   # Format
cargo clippy                # Linter
cargo clippy --fix          # Auto-fix (careful)
```

### Documentation

```bash
cargo doc
cargo doc --open
```

### Dependencies

```bash
cargo add <crate>
cargo update
cargo tree
```

## 3) Clippy “AAA” levels

### Basic

```bash
cargo clippy
```

### Strict (pedantic)

```bash
cargo clippy -- -W clippy::pedantic
```

### Experimental (nursery)

```bash
cargo clippy -- -W clippy::nursery
```

### Deny warnings

```bash
cargo clippy -- -D warnings
```

### AAA check (recommended)

```bash
cargo clippy --all-features --all-targets -- \
  -W clippy::pedantic \
  -W clippy::nursery
```

## 4) Common Clippy fixes

### Unreadable literals

```rust
// ❌ Before
let x = 4000000;

// ✅ After
let x = 4_000_000;
```

### Unnecessary clones

```rust
// ❌ Before
let s = my_string.clone();

// ✅ After (if possible)
let s = &my_string;
```

### Missing docs

```rust
// ❌ Before
pub fn calculate(x: i32) -> i32 { x * 2 }

// ✅ After
/// Multiplies the input by 2
pub fn calculate(x: i32) -> i32 { x * 2 }
```

## 5) GTK4 development (Linux)

### Debian/Ubuntu dependencies

```bash
sudo apt update
sudo apt install libgtk-4-dev build-essential policykit-1
```

### GTK debugging

```bash
G_MESSAGES_DEBUG=all battery-manager --debug
```

### Battery Manager logs: colors (optional)

By default, color output follows the terminal (auto).

```bash
# Force colors (even if stdout isn't a TTY)
BATTERY_MANAGER_COLOR=always battery-manager --debug

# Disable all colors (standard)
NO_COLOR=1 battery-manager --debug
```

## 6) Useful system tools (project)

### Directory tree

```bash
tree -L 2 -I 'target' --dirsfirst
```

### Count lines of code

```bash
cloc src/ --quiet
```

### Fast search (ripgrep)

```bash
rg "pattern" -n src/
```

## 7) systemd: restore service

```bash
systemctl status battery-manager.service
systemctl restart battery-manager.service
journalctl -u battery-manager.service -b --no-pager
```

## 8) pkexec / PolicyKit

The project uses `pkexec` to get root privileges when writing to `/sys/class/power_supply/`.

Quick check:

```bash
pkexec --version
```

## 9) sysfs: threshold compatibility checks

```bash
ls /sys/class/power_supply/BAT0/
cat /sys/class/power_supply/BAT0/status

# Examples (vendor-dependent)
cat /sys/class/power_supply/BAT0/charge_control_end_threshold 2>/dev/null || true
cat /sys/class/power_supply/BAT0/charge_stop_threshold 2>/dev/null || true
```

## 10) Packaging (.deb)

```bash
cd install
./build-deb.sh
```

## 11) Essential workflow

```bash
# Dev cycle
cargo check
cargo run

# Before PR / release
cargo fmt
cargo clippy --all-features --all-targets -- -W clippy::pedantic -W clippy::nursery
cargo test
cargo build --release
```

## 12) Cargo.toml example

```toml
[package]
name = "battery_manager"
version = "1.0.0"
edition = "2021"

[dependencies]
gtk4 = "0.10"
glib = "0.21"
dirs = "6.0"
libc = "0.2"

[profile.release]
opt-level = 3
lto = true
```

<!-- END:EN -->
