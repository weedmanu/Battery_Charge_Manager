# Battery Manager

Gestionnaire de seuils de charge pour batteries Linux avec interface GTK4.

## Description

Application simple permettant de visualiser les informations de batterie et de configurer les seuils de charge sur les ordinateurs portables Linux compatibles. Écrit en Rust, utilise l'interface sysfs du noyau pour lire et modifier les paramètres de charge.

## Ce que fait cette application

- Affiche les informations de base de la batterie (charge, état, santé)
- Permet de définir des seuils de début et fin de charge
- Sauvegarde les seuils dans `/etc/battery-manager/BAT*.conf` (format texte)
- Restaure les seuils au démarrage via un service systemd
- Écrit directement dans `/sys/class/power_supply/` (nécessite pkexec)

## Prérequis

- Debian et dérivés (Ubuntu, Mint, Pop!\_OS, etc.)
- Rust (édition 2021)
- GTK4 et bibliothèques de développement
- pkexec (PolicyKit)
- Un ordinateur portable avec support sysfs pour les seuils de charge

### Installation des dépendances

```bash
sudo apt install libgtk-4-dev build-essential policykit-1
```

## Installation

### Package .deb

```bash
./build-deb.sh
sudo dpkg -i target/battery-manager_1.0.0_amd64.deb
```

### Compilation manuelle

```bash
git clone https://github.com/weedmanu/Battery_Charge_Manager.git
cd Battery_Manager
cargo build --release
sudo ./install.sh
```

Le script d'installation copie le binaire dans `/usr/bin/`, crée l'entrée du menu, et configure le service systemd pour la restauration automatique des seuils.

## Utilisation

Lancer l'application depuis le menu ou en terminal :

```bash
battery-manager
```

L'interface comporte deux onglets :

- **Informations** : affiche l'état de la batterie (charge, santé, voltage, etc.)
- **Paramètres** : curseurs pour définir les seuils de charge (0-100%)

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
│   │   └── traits.rs               # Traits pour injection de dépendances (tests)
│   │
│   └── ui/                         # Interface utilisateur GTK4
│       ├── mod.rs                  # Exports du module ui
│       ├── app.rs                  # Fenêtre principale et gestion onglets
│       ├── components.rs           # Composants réutilisables (cartes d'infos)
│       ├── info_tab.rs             # Onglet "Informations" (affichage batterie)
│       └── settings_tab.rs         # Onglet "Paramètres" (curseurs seuils)
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

## Licence

MIT - voir [LICENSE](LICENSE)
