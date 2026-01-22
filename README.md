# Battery Manager

Gestionnaire de seuils de charge pour batteries Linux avec interface GTK4.

![Battery Manager](resources/icon.png)

## ✨ Fonctionnalités

### 🔋 Gestion de batterie

- **Visualisation en temps réel** : charge, santé, voltage, puissance, cycles
- **Configuration des seuils** : début et fin de charge (0-100%)
- **Sauvegarde persistante** : restauration automatique au démarrage via systemd
- **Support multi-fabricants** : ASUS, Lenovo, Dell, Huawei, Samsung, System76, Tuxedo

### 🌍 Internationalisation

- **Bilingue** : Français et Anglais
- **Détection automatique** : basée sur la langue système ($LANG)
- **Switch en temps réel** : changement de langue dans l'interface
- **Configuration sauvegardée** : `~/.config/battery-manager/language.conf`

### 🎨 Thèmes

- **Thème clair** : nuances de gris (#e6e6e6 - #fcfcfc)
- **Thème sombre** : nuances de gris (#252525 - #424242)
- **Application instantanée** : changement de thème sans redémarrage
- **Configuration sauvegardée** : `~/.config/battery-manager/theme.conf`
- **Sans blanc/noir pur** : design confortable pour les yeux

### 🏗️ Architecture

- **SOLID** : séparation stricte core/ (logique) et ui/ (présentation)
- **36 tests unitaires** : 100% de réussite
- **Documentation complète** : docstrings Rust standard
- **Qualité AAA** : cargo fmt, clippy, tests automatisés

## Description

Application simple permettant de visualiser les informations de batterie et de configurer les seuils de charge sur les ordinateurs portables Linux compatibles. Écrit en Rust avec GTK4, utilise l'interface sysfs du noyau pour lire et modifier les paramètres de charge.

## Ce que fait cette application

- Affiche les informations de base de la batterie (charge, état, santé)
- Permet de définir des seuils de début et fin de charge
- Sauvegarde les seuils dans `/etc/battery-manager/BAT*.conf` (format texte)
- Restaure les seuils au démarrage via un service systemd
- Écrit directement dans `/sys/class/power_supply/` (nécessite pkexec)

## Prérequis

- Debian et dérivés (Ubuntu, Mint, Pop!\_OS, etc.)
- GTK4 et bibliothèques (installation automatique via .deb)
- pkexec (PolicyKit)
- Un ordinateur portable avec support sysfs pour les seuils de charge

### Pour les utilisateurs finaux

Aucune dépendance à installer manuellement. Le package .deb gère tout automatiquement.

### Pour les développeurs

```bash
# Outils de développement Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dépendances GTK4
sudo apt install libgtk-4-dev build-essential policykit-1
```

## Installation

### 📦 Pour les utilisateurs (recommandé)

Installation complète avec icône, raccourci menu et service systemd :

```bash
# Télécharger le .deb depuis les releases GitHub
sudo dpkg -i battery-manager_1.0.0_amd64.deb
```

Le package .deb installe automatiquement :

- ✅ Le binaire exécutable
- ✅ L'icône et le raccourci dans le menu
- ✅ Le service systemd (restauration auto au démarrage)
- ✅ Toutes les dépendances nécessaires

### 🔧 Pour les développeurs

Installation manuelle du service uniquement (sans raccourci menu) :

```bash
git clone https://github.com/weedmanu/Battery_Charge_Manager.git
cd Battery_Manager
cargo build --release
sudo ./install.sh      # Installe uniquement le service systemd
```

**Note** : `install.sh` est conçu pour le développement. Il n'installe PAS le raccourci menu ni l'icône. Pour une installation complète utilisateur, utilisez le package .deb.

Le script d'installation copie le binaire dans `/usr/bin/`, crée l'entrée du menu, et configure le service systemd pour la restauration automatique des seuils.

## Utilisation

Lancer l'application depuis le menu ou en terminal :

```bash
battery-manager                # Lance avec langue système
battery-manager --lang=fr      # Force le français
battery-manager --lang=en      # Force l'anglais
battery-manager --debug        # Active les logs de debug
battery-manager --help         # Affiche l'aide
```

L'interface comporte trois onglets :

- **📊 Informations** : état de la batterie (charge, santé, voltage, puissance, cycles)
- **⚙️ Réglages** : curseurs pour les seuils de charge, alarme, service systemd
- **🎨 Interface** : choix de la langue (FR/EN) et du thème (clair/sombre)

Les seuils sont appliqués immédiatement et sauvegardés pour la prochaine session.

## Désinstallation

```bash
sudo ./uninstall.sh
```

## Structure du projet

```
Battery_Manager/
├── Cargo.toml                      # Dépendances Rust et configuration du projet
├── Cargo.lock                      # Versions exactes des dépendances (généré)
├── LICENSE                         # Licence MIT
├── README.md                       # Documentation (Markdown)
├── README.html                     # Documentation (HTML pour navigation web)
├── CARGO_BASICS.md                 # Documentation sur Cargo
│
├── install.sh                      # Script d'installation manuelle
├── uninstall.sh                    # Script de désinstallation manuelle
├── build-deb.sh                    # Générateur de package .deb
│
├── resources/                      # Fichiers de ressources système
│   ├── battery-manager.desktop     # Entrée du menu applications
│   ├── battery-manager.service     # Service systemd (restaure les seuils au boot)
│   ├── battery-manager-restore.sh  # Script appelé par systemd au démarrage
│   └── icon.png                    # Icône de l'application (8.4KB)
│
├── src/                            # Code source Rust
│   ├── main.rs                     # Point d'entrée, lance l'application GTK
│   │
│   ├── core/                       # Logique métier (indépendante de l'UI)
│   │   ├── mod.rs                  # Exports du module core
│   │   ├── battery.rs              # Lecture infos batterie depuis /sys/class/power_supply/
│   │   ├── power_supply.rs         # Détection alimentation secteur (AC)
│   │   ├── vendor_detection.rs     # Détection fabricant et chemins sysfs spécifiques
│   │   ├── traits.rs               # Traits pour injection de dépendances (tests)
│   │   ├── i18n.rs                 # Système de traduction FR/EN (80+ clés)
│   │   └── debug.rs                # Macros de debug (debug!, debug_ui!, debug_core!)
│   │
│   └── ui/                         # Interface utilisateur GTK4
│       ├── mod.rs                  # Exports du module ui
│       ├── app.rs                  # Fenêtre principale et gestion onglets
│       ├── components.rs           # Composants réutilisables (cartes d'infos)
│       ├── info_tab.rs             # Onglet "Informations" (affichage batterie)
│       ├── settings_tab.rs         # Onglet "Réglages" (curseurs seuils, alarme)
│       ├── ui_tab.rs               # Onglet "Interface" (langue, thème)
│       └── theme.rs                # Gestion des thèmes CSS (clair/sombre)
│
└── target/                         # Dossier de build Cargo (ignoré par git)
    ├── debug/                      # Binaires de développement
    └── release/                    # Binaires optimisés
        └── battery_manager         # Exécutable final
```

### Fichiers de configuration système

Après installation, l'application crée :

- `/etc/battery-manager/BAT*.conf` : seuils configurés (format texte : `START_THRESHOLD=60\nSTOP_THRESHOLD=80`)
- `/usr/bin/battery-manager` : binaire exécutable
- `/usr/bin/battery-manager-restore` : script de restauration
- `/lib/systemd/system/battery-manager.service` : service systemd
- `/usr/share/applications/battery-manager.desktop` : lanceur menu
- `/usr/share/pixmaps/battery-manager.png` : icône

### Fichiers de configuration utilisateur

L'application sauvegarde les préférences dans :

- `~/.config/battery-manager/language.conf` : langue choisie (fr/en)
- `~/.config/battery-manager/theme.conf` : thème choisi (light/dark)

## Compatibilité

L'application fonctionne sur les ordinateurs portables dont le noyau Linux expose les fichiers de contrôle de charge dans `/sys/class/power_supply/BAT*/`.

Chemins supportés :

- `charge_control_start_threshold` / `charge_control_end_threshold`
- `charge_start_threshold` / `charge_stop_threshold`
- `charge_end_threshold`

Fonctionnera si votre ordinateur portable expose ces fichiers via sysfs. Vérifiez avec `ls /sys/class/power_supply/BAT0/` pour voir ce qui est disponible sur votre système.

## Limitations

- Nécessite un support matériel et noyau approprié
- Certains fabricants n'exposent qu'un seul seuil (fin de charge uniquement)
- L'application ne peut pas créer de support là où le matériel/noyau ne le fournit pas
- Le changement de langue nécessite un redémarrage de l'application

## Tests

L'application dispose de 36 tests unitaires :

```bash
cargo test                    # Exécuter tous les tests
cargo test --test-threads=1   # Exécuter en séquentiel (recommandé pour i18n)
```

## Documentation

Générer la documentation du code :

```bash
cargo doc --open              # Génère et ouvre la doc HTML
```

## Qualité du code

```bash
cargo fmt                     # Formater le code
cargo clippy -- -D warnings   # Analyser le code (0 warning)
cargo build --release         # Compiler en mode release
```

## Licence

MIT - voir [LICENSE](LICENSE)
