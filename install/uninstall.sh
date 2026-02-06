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
rm -f /usr/bin/battery-manager
rm -f /usr/bin/battery-manager-restore
rm -f /lib/systemd/system/battery-manager.service
rm -f /usr/share/applications/battery-manager.desktop
rm -rf /usr/share/battery-manager

# Recharger systemd
echo "Rechargement de systemd..."
systemctl daemon-reload

echo -e "\n${GREEN}✓ Désinstallation terminée avec succès!${NC}"
echo -e "\n${YELLOW}Note: Les fichiers de configuration dans /etc/battery-manager/ ont été conservés${NC}"
echo -e "Pour les supprimer également: sudo rm -rf /etc/battery-manager/"
echo -e "\n${YELLOW}Note: Ce script désinstalle uniquement le service systemd${NC}"
echo -e "Pour une désinstallation complète du package .deb: sudo apt remove battery-manager"
