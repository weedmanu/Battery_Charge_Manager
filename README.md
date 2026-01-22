# Battery Manager

Un gestionnaire de batterie moderne pour Linux avec interface GTK4, permettant de contrôler les seuils de charge sans dépendance à TLP.

## Fonctionnalités

- 🔋 **Affichage complet des informations de batterie** : état, charge, santé, capacité, paramètres électriques
- ⚙️ **Contrôle des seuils de charge** : définir les seuils de début et fin de charge pour préserver la batterie
- 💾 **Persistance automatique** : les seuils sont restaurés automatiquement au démarrage via systemd
- 🎨 **Interface moderne** : GTK4 avec organisation en cartes compactes
- 🏗️ **Architecture SOLID** : séparation claire entre logique métier (core) et interface (ui)
- 🔓 **Aucune dépendance TLP** : écriture directe dans les fichiers système `/sys/class/power_supply/`

## Prérequis

- Rust (édition 2021 ou plus récente)
- GTK4 et bibliothèques de développement
- pkexec (PolicyKit) pour les opérations privilégiées

### Installation des dépendances (Debian/Ubuntu)

```bash
sudo apt install libgtk-4-dev build-essential policykit-1
```

### Installation des dépendances (Fedora)

```bash
sudo dnf install gtk4-devel gcc polkit
```

### Installation des dépendances (Arch)

```bash
sudo pacman -S gtk4 base-devel polkit
```

## Installation

### Option 1 : Installation via package .deb (Recommandé)

1. **Télécharger le package .deb** depuis les [Releases](https://github.com/votre-utilisateur/Battery_Manager/releases)

2. **Installer le package** :
   ```bash
   sudo dpkg -i battery-manager_1.0.0_amd64.deb
   sudo apt-get install -f  # Si des dépendances manquent
   ```

### Option 2 : Compilation depuis les sources

1. **Cloner le dépôt** :

   ```bash
   git clone https://github.com/votre-utilisateur/Battery_Manager.git
   cd Battery_Manager
   ```

2. **Compiler et installer** :

   ```bash
   cargo build --release
   sudo ./install.sh
   ```

### Option 3 : Créer votre propre package .deb

```bash
./build-deb.sh
sudo dpkg -i target/battery-manager_1.0.0_amd64.deb
```

Le script d'installation :

- Compile le projet en mode release
- Copie le binaire vers `/usr/local/bin/battery-manager`
- Installe le script de restauration `/usr/local/bin/battery-manager-restore`
- Crée l'entrée du menu applications `/usr/share/applications/battery-manager.desktop`
- Configure le service systemd `/etc/systemd/system/battery-manager.service`
- Active le service pour la restauration automatique au démarrage

## Utilisation

### Lancer l'application

Depuis le menu des applications ou en ligne de commande :

```bash
battery-manager
```

### Onglet Informations

Affiche 9 cartes avec toutes les informations de votre batterie :

- État d'alimentation (sur secteur / sur batterie)
- État actuel (en charge, décharge, pleine)
- Niveau de charge et capacité
- Santé de la batterie
- Paramètres électriques (tension, courant, puissance)
- Informations système (fabricant, modèle, technologie)
- Seuils de charge configurés

### Onglet Paramètres

- **Définir les seuils de charge** : utilisez les curseurs pour choisir entre 0% et 100%
  - Seuil de début : le niveau auquel la batterie commence à se charger
  - Seuil de fin : le niveau maximal de charge
- **Appliquer les paramètres** : sauvegarde et applique immédiatement les seuils
- **Charger à 100%** : désactive temporairement les seuils pour une charge complète

### Gestion du service systemd

```bash
# Voir l'état du service
sudo systemctl status battery-manager

# Restaurer les seuils manuellement
sudo /usr/local/bin/battery-manager-restore

# Désactiver la restauration automatique
sudo systemctl disable battery-manager

# Réactiver la restauration automatique
sudo systemctl enable battery-manager
```

## Désinstallation

### Si installé via .deb

```bash
# Désinstaller en gardant la configuration
sudo apt remove battery-manager

# Désinstaller et supprimer toute la configuration
sudo apt purge battery-manager
```

### Si installé via install.sh

```bash
sudo ./uninstall.sh
```

Les fichiers de configuration dans `/etc/battery-manager/` peuvent être supprimés manuellement si nécessaire.

## Architecture du projet

```
Battery_Manager/
├── src/
│   ├── main.rs              # Point d'entrée
│   ├── core/                # Logique métier
│   │   ├── mod.rs
│   │   ├── battery.rs       # Lecture des informations de batterie
│   │   ├── power_supply.rs  # Détection de l'alimentation secteur
│   │   └── battery_control.rs # Gestion des seuils de charge
│   └── ui/                  # Interface utilisateur
│       ├── mod.rs
│       ├── app.rs           # Fenêtre principale
│       ├── components.rs    # Composants réutilisables
│       ├── info_tab.rs      # Onglet d'informations
│       └── settings_tab.rs  # Onglet des paramètres
├── resources/
│   ├── battery-manager.desktop      # Entrée du menu
│   ├── battery-manager.service      # Service systemd
│   └── battery-manager-restore.sh   # Script de restauration
├── install.sh               # Script d'installation
├── uninstall.sh            # Script de désinstallation
└── Cargo.toml              # Dépendances Rust
```

## Compatibilité

Cette application fonctionne avec les ordinateurs portables qui exposent les seuils de charge via sysfs. Les chemins suivants sont supportés :

- `/sys/class/power_supply/BAT*/charge_control_start_threshold`
- `/sys/class/power_supply/BAT*/charge_control_end_threshold`
- `/sys/class/power_supply/BAT*/charge_start_threshold`
- `/sys/class/power_supply/BAT*/charge_stop_threshold`
- `/sys/class/power_supply/BAT*/charge_end_threshold`

Testé sur :

- ThinkPad (Lenovo)
- Autres ordinateurs portables avec support du noyau Linux

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :

- Signaler des bugs
- Proposer de nouvelles fonctionnalités
- Améliorer la documentation
- Ajouter le support pour d'autres fabricants

## Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

## Remerciements

- GTK Team pour GTK4
- Communauté Rust pour les excellentes bibliothèques
- Utilisateurs de TLP pour l'inspiration
