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
if [ ! -f "target/release/battery_manager" ]; then
    echo "Le binaire n'existe pas. Veuillez compiler le projet en tant qu'utilisateur normal :"
    echo "  cargo build --release"
    echo "Puis relancez ce script d'installation avec sudo."
    exit 1
fi

echo "Utilisation du binaire existant..."

# Créer les répertoires nécessaires
mkdir -p /usr/local/bin
mkdir -p /usr/share/applications
mkdir -p /usr/share/pixmaps
mkdir -p /etc/systemd/system

# Copier le binaire
echo "Installation du binaire..."
cp target/release/battery_manager /usr/local/bin/battery-manager
chmod +x /usr/local/bin/battery-manager

# Copier l'icône
echo "Installation de l'icône..."
cp resources/icon.png /usr/share/pixmaps/battery-manager.png

# Copier le script de restauration
echo "Installation du script de restauration..."
cp resources/battery-manager-restore.sh /usr/local/bin/battery-manager-restore
chmod +x /usr/local/bin/battery-manager-restore

# Copier le fichier .desktop
echo "Installation du fichier .desktop..."
cp resources/battery-manager.desktop /usr/share/applications/

# Copier le service systemd
echo "Installation du service systemd..."
cp resources/battery-manager.service /etc/systemd/system/

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
echo -e "  - Restaurer maintenant: sudo /usr/local/bin/battery-manager-restore"
