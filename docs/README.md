<!-- BEGIN:FR -->

# Battery Manager

> Gestionnaire de seuils de charge pour batteries Linux avec interface GTK4

![Battery Manager](icon.png)

[![Rust](https://img.shields.io/badge/Rust-1.92.0-orange.svg)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GTK-4.0-blue.svg)](https://gtk.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-40%2F40_passing-success.svg)](#tests)
[![Quality](https://img.shields.io/badge/Clippy-0_warnings-success.svg)](#qualit%C3%A9-aaa)
[![Code](https://img.shields.io/badge/Lines-3473_Rust-blueviolet.svg)](#structure-du-projet)

---

## ✨ Fonctionnalités

### 🔋 Gestion de batterie

- **Visualisation en temps réel** : charge, santé, voltage, puissance, cycles
- **Configuration des seuils** : début et fin de charge (0-100%)
- **Persistance optionnelle** : restauration au démarrage via systemd (si activé)
- **Support multi-fabricants** : ASUS, Lenovo, Dell, Huawei, Samsung, System76, Tuxedo

### 🌍 Internationalisation

- **Bilingue** : Français et Anglais (80+ clés de traduction)
- **Détection automatique** : basée sur la langue système (`$LANG`)
- **Switch en temps réel** : changement de langue dans l'interface
- **Configuration persistante** : `~/.config/battery-manager/language.conf`

### 🎨 Thèmes

- **Thème clair** : nuances de gris (#f6f5f4 - #ffffff)
- **Thème sombre** : nuances de gris (#252525 - #424242)
- **Application instantanée** : changement de thème sans redémarrage
- **Configuration persistante** : `~/.config/battery-manager/theme.conf`
- **Design confortable** : sans blanc/noir pur pour le confort visuel

### 🏗️ Architecture

- **SOLID** : séparation stricte core/ (logique) et ui/ (présentation)
- **40 tests unitaires** : 100% de réussite
- **Documentation complète** : docstrings Rust standard
- **Qualité AAA** : 0 warning Clippy (pedantic + nursery)

---

## 📝 Description

Application simple permettant de visualiser les informations de batterie et de configurer les seuils de charge sur les ordinateurs portables Linux compatibles.

**Technologies** : Écrit en Rust avec GTK4, utilise l'interface sysfs du noyau Linux pour lire et modifier les paramètres de charge.

### Ce que fait cette application

- ✅ Affiche les informations de base de la batterie (charge, état, santé, cycles)
- ✅ Permet de définir des seuils de début et fin de charge (prolonge la durée de vie de la batterie)
- ✅ Sauvegarde les seuils dans `/etc/battery-manager/BAT*.conf` (format texte simple)
- ✅ Restaure les seuils au démarrage via un service systemd
- ✅ Écrit directement dans `/sys/class/power_supply/` (nécessite pkexec)
- ✅ Détecte les batteries de périphériques (souris/clavier sans fil)

---

## 📦 Installation

### Pour les utilisateurs (recommandé)

Installation complète avec icône, raccourci menu et service systemd :

```bash
# Télécharger le .deb depuis les releases GitHub
sudo dpkg -i battery-manager_0.9.8_amd64.deb
```

**Le package .deb installe automatiquement :**

- ✅ Le binaire exécutable (`/usr/bin/battery-manager`)
- ✅ L'icône et le raccourci dans le menu applications
- ✅ Le service systemd (restauration auto au démarrage)
- ✅ Toutes les dépendances GTK4 nécessaires

### Pour les développeurs

Installation manuelle du service uniquement (sans raccourci menu) :

```bash
git clone https://github.com/weedmanu/Battery_Charge_Manager.git
cd Battery_Manager
cargo build --release
cd install
sudo ./install.sh      # Installe uniquement le service systemd
```

> **Note** : `install.sh` est conçu pour le développement. Il n'installe PAS le raccourci menu ni l'icône. Pour une installation complète utilisateur, utilisez le package `.deb`.

---

## 🚀 Utilisation

Lancer l'application depuis le menu ou en terminal :

```bash
battery-manager                # Lance avec langue système
battery-manager --lang=fr      # Force le français
battery-manager --lang=en      # Force l'anglais
battery-manager --debug        # Active les logs de debug
battery-manager --help         # Affiche l'aide complète

# Forcer/désactiver les couleurs des logs (optionnel)
BATTERY_MANAGER_COLOR=always battery-manager --debug
NO_COLOR=1 battery-manager --debug
```

### Interface

L'interface comporte **4 onglets** :

- **📊 Informations** : état de la batterie (charge, santé, voltage, puissance, cycles)
- **🖱️ Périphériques** : batteries externes (souris, clavier sans fil)
- **⚙️ Réglages** : curseurs pour les seuils de charge, alarme, activation service systemd
- **🎨 Interface** : choix de la langue (FR/EN) et du thème (clair/sombre)

Les seuils sont appliqués **immédiatement**. Ils sont restaurés au prochain démarrage uniquement si le service systemd est activé ; sinon, ils seront perdus après redémarrage.

---

## 🧩 Scripts et fonctionnement (important)

### 1) Où sont stockés les seuils ?

Les seuils sont sauvegardés dans des fichiers texte :

- `/etc/battery-manager/BAT*.conf`

Format attendu (exemple) :

```text
START_THRESHOLD=60
STOP_THRESHOLD=80
```

Notes :

- `STOP_THRESHOLD` est requis.
- `START_THRESHOLD` est **optionnel** : certains laptops ne supportent pas un seuil de début.

### 2) Comment les seuils sont restaurés au boot ?

Le service systemd `battery-manager.service` exécute un script one-shot :

- `/usr/bin/battery-manager-restore`

Ce script lit `/etc/battery-manager/*.conf` puis tente d'écrire dans sysfs.

Seuil de début (si défini) :

- `/sys/class/power_supply/BAT*/charge_control_start_threshold`
- `/sys/class/power_supply/BAT*/charge_start_threshold`

Seuil de fin :

- `/sys/class/power_supply/BAT*/charge_control_end_threshold`
- `/sys/class/power_supply/BAT*/charge_stop_threshold`
- `/sys/class/power_supply/BAT*/charge_end_threshold`

Le script essaie ces chemins dans l'ordre et applique le premier fichier disponible/inscriptible.

Commandes utiles :

```bash
systemctl status battery-manager.service
sudo battery-manager-restore
journalctl -u battery-manager.service -b --no-pager
```

### 3) À quoi servent les scripts dans `install/` ?

- `install/install.sh` : installe **binaire + restore script + service systemd** (mode dev, sans menu/icon)
- `install/uninstall.sh` : supprime ces fichiers (conserve `/etc/battery-manager/`)
- `install/build-deb.sh` : construit le package `.deb` (installation utilisateur complète)

---

## 🗑️ Désinstallation

```bash
# Via .deb
sudo apt remove battery-manager

# Via script manuel
cd Battery_Manager/install
sudo ./uninstall.sh
```

---

## 📂 Structure du projet

```
Battery_Manager/                        # 7 directories, 35 files
├── Cargo.lock
├── Cargo.toml
├── docs
│   ├── generate_docs.py
│   ├── icon.png
│   ├── README.html
│   ├── README.md
│   ├── REFERENCES.html
│   ├── REFERENCES.md
│   └── style.css
├── install
│   ├── build-deb.sh
│   ├── install.sh
│   └── uninstall.sh
├── LICENSE
├── README.md
├── resources
│   ├── battery-manager.desktop
│   ├── battery-manager-restore.sh
│   ├── battery-manager.service
│   └── icon.png
└── src                                 # Code source Rust (3473 lignes, 17 fichiers)
    ├── core
    │   ├── battery.rs
    │   ├── debug.rs
    │   ├── i18n.rs
    │   ├── mod.rs
    │   ├── peripheral.rs
    │   ├── power_supply.rs
    │   ├── traits.rs
    │   └── vendor_detection.rs
    ├── main.rs
    └── ui
        ├── app.rs
        ├── components.rs
        ├── info_tab.rs
        ├── mod.rs
        ├── peripherals_tab.rs
        ├── settings_tab.rs
        ├── theme.rs
        └── ui_tab.rs
```

### Statistiques du code

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            17            564            618           3473
-------------------------------------------------------------------------------
```

### Fichiers de configuration système

Après installation, l'application crée :

- `/etc/battery-manager/BAT*.conf` : seuils configurés (`START_THRESHOLD=60\nSTOP_THRESHOLD=80`)
- `/usr/bin/battery-manager` : binaire exécutable
- `/usr/bin/battery-manager-restore` : script de restauration
- `/lib/systemd/system/battery-manager.service` : service systemd
- `/usr/share/applications/battery-manager.desktop` : lanceur menu
- `/usr/share/pixmaps/battery-manager.png` : icône

### Fichiers de configuration utilisateur

L'application sauvegarde les préférences dans `~/.config/battery-manager/` :

- `language.conf` : langue choisie (`fr` ou `en`)
- `theme.conf` : thème choisi (`light` ou `dark`)

---

## 🔧 Prérequis

### Pour les utilisateurs finaux

- **OS** : Debian et dérivés (Ubuntu, Mint, Pop!\_OS, etc.)
- **GTK4** : installation automatique via le package .deb
- **pkexec** : PolicyKit (généralement préinstallé)
- **Matériel** : ordinateur portable avec support sysfs pour les seuils de charge

### Pour les développeurs

```bash
# Outils de développement Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dépendances GTK4
sudo apt install libgtk-4-dev build-essential policykit-1
```

---

## ✅ Compatibilité

L'application fonctionne sur les ordinateurs portables dont le **noyau Linux expose les fichiers de contrôle de charge** dans `/sys/class/power_supply/BAT*/`.

### Chemins supportés

- `charge_control_start_threshold` / `charge_control_end_threshold` (ASUS, Huawei)
- `charge_start_threshold` / `charge_stop_threshold` (Lenovo, Samsung)
- `charge_end_threshold` (Dell, System76)

### Vérification de compatibilité

```bash
# Vérifier les fichiers disponibles
ls /sys/class/power_supply/BAT0/

# Tester l'écriture (nécessite root)
echo 80 | sudo tee /sys/class/power_supply/BAT0/charge_control_end_threshold
```

Fonctionnera si votre ordinateur portable expose ces fichiers via sysfs.

---

## ⚠️ Limitations

- ❌ Nécessite un support matériel et noyau approprié
- ❌ Certains fabricants n'exposent qu'un seul seuil (fin de charge uniquement)
- ❌ L'application ne peut pas créer de support là où le matériel/noyau ne le fournit pas
- ❌ Le changement de langue nécessite un redémarrage de l'application

---

## 🧪 Tests

L'application dispose de **40 tests unitaires** avec **100% de réussite** :

```bash
cargo test                    # Exécuter tous les tests
cargo test --test-threads=1   # Exécuter en séquentiel (recommandé pour i18n)

# Résultat attendu :
# test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Couverture des tests

- ✅ **Validation des entrées** : noms de batterie, seuils invalides
- ✅ **Calculs** : santé, usure, puissance, conversions (mAh, V, mA, W)
- ✅ **Détection fabricant** : ASUS, Lenovo, Dell, Huawei, Samsung, System76, Tuxedo
- ✅ **Internationalisation** : traductions FR/EN, clés manquantes
- ✅ **Traits et mocks** : injection de dépendances pour les tests

---

## 📚 Documentation

Générer la documentation du code :

```bash
cargo doc --open              # Génère et ouvre la doc HTML dans le navigateur
```

La documentation inclut :

- ✅ Toutes les fonctions publiques avec docstrings
- ✅ Exemples d'utilisation
- ✅ Structure des modules
- ✅ Graphes de dépendances

---

## 🏆 Qualité AAA

### Analyse statique

```bash
cargo fmt                     # Formater le code (style officiel Rust)
cargo clippy --all-features --all-targets -- -W clippy::pedantic -W clippy::nursery
                              # Analyse stricte : 0 warning ✅

cargo build --release         # Compiler en mode release
```

### Métriques de qualité

| Métrique            | Valeur                      | Status        |
| ------------------- | --------------------------- | ------------- |
| **Warnings Clippy** | 0 / 0                       | ✅ 100%       |
| **Tests unitaires** | 40 / 40                     | ✅ 100%       |
| **Documentation**   | 100%                        | ✅ Complète   |
| **Sécurité**        | `unwrap()` limité aux tests | ✅ Hardened   |
| **Architecture**    | SOLID                       | ✅ Clean Code |

### Optimisations appliquées

- ✅ **0 warnings Clippy** (pedantic + nursery)
- ✅ **Validation stricte** : noms de batterie, seuils, chemins de fichiers
- ✅ **Gestion d'erreurs** : `Result<T, E>` partout, `expect()` avec messages descriptifs
- ✅ **Séparation des préoccupations** : core/ sans dépendances GTK
- ✅ **Injection de dépendances** : traits pour testabilité
- ✅ **Code factorisé** : helpers réutilisables (716 lignes économisées)

---

## 📄 Licence

MIT - voir [LICENSE](LICENSE)

---

## 🤝 Contribution

Les contributions sont les bienvenues ! Pour contribuer :

1. Fork le projet
2. Créer une branche (`git checkout -b feature/amélioration`)
3. Commit les changements (`git commit -m 'Ajout fonctionnalité'`)
4. Push vers la branche (`git push origin feature/amélioration`)
5. Ouvrir une Pull Request

### Guidelines

- ✅ Respecter `cargo fmt` et `cargo clippy`
- ✅ Ajouter des tests pour les nouvelles fonctionnalités
- ✅ Documenter les fonctions publiques
- ✅ Mettre à jour le README si nécessaire

---

## 📞 Support

- **Issues** : [GitHub Issues](https://github.com/weedmanu/Battery_Charge_Manager/issues)
- **Discussions** : [GitHub Discussions](https://github.com/weedmanu/Battery_Charge_Manager/discussions)

---

**Made with ❤️ in Rust** | Version 0.9.8

<!-- END:FR -->

---

<!-- BEGIN:EN -->

# Battery Manager

> Battery charge threshold manager for Linux laptops with a GTK4 UI

![Battery Manager](icon.png)

[![Rust](https://img.shields.io/badge/Rust-1.92.0-orange.svg)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GTK-4.0-blue.svg)](https://gtk.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-40%2F40_passing-success.svg)](#tests)
[![Quality](https://img.shields.io/badge/Clippy-0_warnings-success.svg)](#aaa-quality)
[![Code](https://img.shields.io/badge/Lines-3473_Rust-blueviolet.svg)](#project-structure)

---

## ✨ Features

### 🔋 Battery management

- **Real-time view**: charge, health, voltage, power, cycles
- **Threshold configuration**: start/stop charge (0-100%)
- **Optional persistence**: restored at boot via systemd (if enabled)
- **Multi-vendor support**: ASUS, Lenovo, Dell, Huawei, Samsung, System76, Tuxedo

### 🌍 Internationalization

- **Bilingual**: French + English (80+ translation keys)
- **Auto-detect**: based on system language (`$LANG`)
- **Live switch**: change language from the UI
- **Persistent config**: `~/.config/battery-manager/language.conf`

### 🎨 Themes

- **Light theme**: gray palette (#f6f5f4 - #ffffff)
- **Dark theme**: gray palette (#252525 - #424242)
- **Instant apply**: no restart required
- **Persistent config**: `~/.config/battery-manager/theme.conf`
- **Eye-friendly**: avoids pure white/black

### 🏗️ Architecture

- **SOLID**: strict separation between core/ (logic) and ui/ (presentation)
- **40 unit tests**: 100% passing
- **Complete docs**: standard Rust docstrings
- **AAA quality**: 0 Clippy warnings (pedantic + nursery)

---

## 📝 Description

Simple desktop app to view battery information and configure charge thresholds on supported Linux laptops.

**Tech**: written in Rust + GTK4, uses Linux sysfs to read/write charging parameters.

### What this app does

- ✅ Displays key battery info (charge, status, health, cycles)
- ✅ Lets you set start/stop thresholds (battery longevity)
- ✅ Saves thresholds to `/etc/battery-manager/BAT*.conf` (simple text format)
- ✅ Restores thresholds at boot via a systemd service
- ✅ Writes to `/sys/class/power_supply/` (needs pkexec)
- ✅ Detects peripheral batteries (wireless mouse/keyboard)

---

## 📦 Installation

### End users (recommended)

Full install with menu entry, icon, and systemd service:

```bash
sudo dpkg -i battery-manager_0.9.8_amd64.deb
```

### Developers

Manual service install (no menu entry/icon):

```bash
git clone https://github.com/weedmanu/Battery_Charge_Manager.git
cd Battery_Manager
cargo build --release
cd install
sudo ./install.sh
```

> Note: `install.sh` is developer-oriented. For a full user install, use the `.deb` package.

---

## 🚀 Usage

Launch from the app menu or from a terminal:

```bash
battery-manager
battery-manager --lang=fr
battery-manager --lang=en
battery-manager --debug
battery-manager --help

# Optional: force/disable log colors
BATTERY_MANAGER_COLOR=always battery-manager --debug
NO_COLOR=1 battery-manager --debug
```

### UI

The UI has **4 tabs**:

- **📊 Information**: charge/health/voltage/power/cycles
- **🖱️ Peripherals**: external batteries (mouse/keyboard)
- **⚙️ Settings**: thresholds, alarm, systemd toggle
- **🎨 Interface**: language + theme

Thresholds are applied immediately. They are restored at the next boot only if the systemd service is enabled; otherwise, they will be lost after reboot.

---

## 🧩 Scripts & behavior (important)

### 1) Where are thresholds stored?

Thresholds are saved as plain text files:

- `/etc/battery-manager/BAT*.conf`

Expected format (example):

```text
START_THRESHOLD=60
STOP_THRESHOLD=80
```

Notes:

- `STOP_THRESHOLD` is required.
- `START_THRESHOLD` is **optional**: many laptops only support a stop/end threshold.

### 2) How are thresholds restored at boot?

The systemd service `battery-manager.service` runs a one-shot script:

- `/usr/bin/battery-manager-restore`

It reads `/etc/battery-manager/*.conf` and writes to sysfs.

Start threshold (if set):

- `/sys/class/power_supply/BAT*/charge_control_start_threshold`
- `/sys/class/power_supply/BAT*/charge_start_threshold`

Stop threshold:

- `/sys/class/power_supply/BAT*/charge_control_end_threshold`
- `/sys/class/power_supply/BAT*/charge_stop_threshold`
- `/sys/class/power_supply/BAT*/charge_end_threshold`

The script tries these paths in order and uses the first writable file it finds.

Useful commands:

```bash
systemctl status battery-manager.service
sudo battery-manager-restore
journalctl -u battery-manager.service -b --no-pager
```

### 3) What are the scripts in `install/`?

- `install/install.sh`: installs **binary + restore script + systemd service** (dev mode, no menu/icon)
- `install/uninstall.sh`: removes those files (keeps `/etc/battery-manager/`)
- `install/build-deb.sh`: builds the `.deb` package (full end-user install)

---

## 🗑️ Uninstall

```bash
sudo apt remove battery-manager

cd Battery_Manager/install
sudo ./uninstall.sh
```

---

## 📂 Project structure

```
Battery_Manager/                        # 7 directories, 16 files
├── Cargo.toml                          # Rust deps (GTK4, serde)
├── Cargo.lock                          # Locked dependency versions
├── LICENSE                             # MIT
├── README.md                           # Main documentation (Markdown)
├── README.html                         # Main documentation (HTML)
├── REFERENCES.md                       # References (Markdown)
├── REFERENCES.html                     # References (HTML)
├── docs/
│   └── style.css                       # Modern docs CSS (menu + theme + FR/EN)
│
├── install/
│   ├── build-deb.sh
│   ├── install.sh
│   └── uninstall.sh
│
├── resources/
│   ├── battery-manager.desktop
│   ├── battery-manager-restore.sh
│   ├── battery-manager.service
│   └── icon.png
│
└── src/                                # Rust source (2728 LOC, 17 files)
    ├── core/
    ├── ui/
    └── main.rs
```

### Code stats

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            17            453            578           2728
-------------------------------------------------------------------------------
```

---

## 🔧 Requirements

### End users

- Debian-based distro (Ubuntu/Mint/Pop!\_OS, ...)
- GTK4 deps are installed via the `.deb`
- pkexec (PolicyKit)
- Hardware/kernel support for charge thresholds

### Developers

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install libgtk-4-dev build-essential policykit-1
```

---

## ✅ Compatibility

Works on laptops exposing threshold controls via `/sys/class/power_supply/BAT*/`.

Supported sysfs names:

- `charge_control_start_threshold` / `charge_control_end_threshold`
- `charge_start_threshold` / `charge_stop_threshold`
- `charge_end_threshold`

---

## ⚠️ Limitations

- Requires proper hardware/kernel support
- Some vendors expose only one threshold
- Language change requires restarting the app

---

## 🧪 Tests

```bash
cargo test
cargo test --test-threads=1
```

Expected:

```text
test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 🏆 AAA Quality

```bash
cargo fmt
cargo clippy --all-features --all-targets -- -W clippy::pedantic -W clippy::nursery
cargo build --release
```

---

## 📄 License

MIT — see [LICENSE](LICENSE)

<!-- END:EN -->
