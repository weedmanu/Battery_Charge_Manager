#!/bin/bash
# Script de désinstallation de Battery Manager

set -e

# Vérifier les privilèges root
if [[ $EUID -ne 0 ]]; then
   echo "Ce script doit être exécuté avec les privilèges root (sudo)" 
   exit 1
fi

# Couleurs pour l'affichage
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Désinstallation de Battery Manager...${NC}\n"

# Désactiver et arrêter le service
echo "Désactivation du service..."
systemctl disable battery-manager.service 2>/dev/null || true
systemctl stop battery-manager.service 2>/dev/null || true

# Supprimer les fichiers
echo "Suppression des fichiers..."
rm -f /usr/local/bin/battery-manager
rm -f /usr/local/bin/battery-manager-restore
rm -f /usr/share/applications/battery-manager.desktop
rm -f /usr/share/pixmaps/battery-manager.png
rm -f /etc/systemd/system/battery-manager.service

# Recharger systemd
echo "Rechargement de systemd..."
systemctl daemon-reload

echo -e "\n${GREEN}✓ Désinstallation terminée avec succès!${NC}"
echo -e "\n${YELLOW}Note: Les fichiers de configuration dans ~/.config/battery-manager/ ont été conservés${NC}"
echo -e "Pour les supprimer également: rm -rf ~/.config/battery-manager/"
