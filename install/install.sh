#!/bin/bash
# Script d'installation de Battery Manager

set -e

# Vérifier les privilèges root
if [[ $EUID -ne 0 ]]; then
   echo "Ce script doit être exécuté avec les privilèges root (sudo)" 
   exit 1
fi

# Couleurs pour l'affichage
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Installation de Battery Manager...${NC}\n"

# Vérifier que le binaire existe ou compiler en tant qu'utilisateur normal
if [ ! -f "../target/release/battery_manager" ]; then
    echo "Le binaire n'existe pas. Veuillez compiler le projet en tant qu'utilisateur normal :"
    echo "  cargo build --release"
    echo "Puis relancez ce script d'installation avec sudo."
    exit 1
fi

echo "Utilisation du binaire existant..."

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Créer les répertoires nécessaires
mkdir -p /usr/bin
mkdir -p /lib/systemd/system

# Copier le binaire
echo "Installation du binaire..."
cp "${PROJECT_ROOT}/target/release/battery_manager" /usr/bin/battery-manager
chmod +x /usr/bin/battery-manager

# Copier le script de restauration
echo "Installation du script de restauration..."
cp "${PROJECT_ROOT}/resources/battery-manager-restore.sh" /usr/bin/battery-manager-restore
chmod +x /usr/bin/battery-manager-restore

# Copier le service systemd
echo "Installation du service systemd..."
cp "${PROJECT_ROOT}/resources/battery-manager.service" /lib/systemd/system/

# Recharger systemd
echo "Rechargement de systemd..."
systemctl daemon-reload

# Activer le service
echo "Activation du service au démarrage..."
systemctl enable battery-manager.service

echo -e "\n${GREEN}✓ Installation terminée avec succès!${NC}"
echo -e "\nPour désinstaller, exécutez: sudo ./uninstall.sh"
echo -e "\nCommandes utiles:"
echo -e "  - Lancer l'application: battery-manager"
echo -e "  - État du service: sudo systemctl status battery-manager"
echo -e "  - Restaurer maintenant: sudo battery-manager-restore"
