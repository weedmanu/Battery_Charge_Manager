#!/bin/bash
# Script de génération du package .deb pour Battery Manager

set -e

# Variables
APP_NAME="battery-manager"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${PROJECT_ROOT}/Cargo.toml")"
ARCH="amd64"
MAINTAINER="Battery Manager Team <battery-manager@example.com>"
DESCRIPTION="Battery charge threshold manager for laptops with peripheral battery support"
PACKAGE_NAME="${APP_NAME}_${VERSION}_${ARCH}"

# Couleurs
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Fonction de nettoyage automatique
cleanup() {
    if [ -d "target/deb" ]; then
        echo -e "\n${YELLOW}Nettoyage du dossier de build...${NC}"
        rm -rf "target/deb"
        echo -e "${GREEN}✓${NC} Dossier de build supprimé"
    fi
}

# Nettoyer automatiquement à la fin du script ou en cas d'erreur
trap cleanup EXIT

echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   Battery Manager - Générateur de package .deb${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}\n"

# Vérifier que le binaire existe
if [ ! -f "${PROJECT_ROOT}/target/release/battery_manager" ]; then
    echo -e "${YELLOW}Compilation du projet...${NC}"
    cd "${PROJECT_ROOT}"
    cargo build --release
    cd "${PROJECT_ROOT}/install"
fi

echo -e "${GREEN}✓${NC} Binaire trouvé\n"

# Créer la structure du package
DEB_DIR="../target/deb/${PACKAGE_NAME}"
rm -rf "../target/deb"
mkdir -p "${DEB_DIR}"

echo -e "${BLUE}Création de la structure du package...${NC}"

# Structure DEBIAN
mkdir -p "${DEB_DIR}/DEBIAN"

# Créer le fichier control
cat > "${DEB_DIR}/DEBIAN/control" << EOF
Package: ${APP_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: ${MAINTAINER}
Description: ${DESCRIPTION}
 Battery Manager est une application GTK4 qui permet de gérer
 les seuils de charge de la batterie sur les ordinateurs portables.
 .
 Fonctionnalités :
  - Affichage des informations de batterie en temps réel
  - Configuration des seuils de charge (début/fin)
  - Gestion de l'alarme de décharge
  - Service systemd pour restaurer les seuils au démarrage
  - Interface graphique moderne en GTK4
Depends: libgtk-4-1, libglib2.0-0
EOF

# Créer le script postinst (post-installation)
cat > "${DEB_DIR}/DEBIAN/postinst" << 'EOF'
#!/bin/bash
set -e

# Activer et démarrer le service systemd
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload
    systemctl enable battery-manager.service
    systemctl start battery-manager.service
fi

# Mettre à jour la base de données des applications
if command -v update-desktop-database > /dev/null 2>&1; then
    update-desktop-database /usr/share/applications || true
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Battery Manager installé avec succès !"
echo ""
echo "Pour lancer l'application :"
echo "  - Depuis le terminal : battery-manager"
echo "  - Depuis le menu : cherchez 'Battery Manager'"
echo ""
echo "Le service systemd est actif et restaurera"
echo "les seuils de charge à chaque démarrage."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit 0
EOF

# Créer le script prerm (pré-désinstallation)
cat > "${DEB_DIR}/DEBIAN/prerm" << 'EOF'
#!/bin/bash
set -e

# Arrêter et désactiver le service systemd
if [ -d /run/systemd/system ]; then
    systemctl stop battery-manager.service || true
    systemctl disable battery-manager.service || true
fi

exit 0
EOF

# Créer le script postrm (post-désinstallation)
cat > "${DEB_DIR}/DEBIAN/postrm" << 'EOF'
#!/bin/bash
set -e

# Recharger systemd
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || true
fi

# Supprimer les fichiers de configuration
if [ "$1" = "purge" ]; then
    rm -rf /etc/battery-manager
    echo "✓ Configuration supprimée"
fi

# Mettre à jour la base de données des applications
if command -v update-desktop-database > /dev/null 2>&1; then
    update-desktop-database /usr/share/applications || true
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Battery Manager désinstallé"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit 0
EOF

# Rendre les scripts exécutables
chmod 755 "${DEB_DIR}/DEBIAN/postinst"
chmod 755 "${DEB_DIR}/DEBIAN/prerm"
chmod 755 "${DEB_DIR}/DEBIAN/postrm"

echo -e "${GREEN}✓${NC} Fichiers DEBIAN créés"

# Créer la structure de fichiers
mkdir -p "${DEB_DIR}/usr/bin"
mkdir -p "${DEB_DIR}/usr/share/applications"
mkdir -p "${DEB_DIR}/usr/share/pixmaps"
mkdir -p "${DEB_DIR}/lib/systemd/system"
mkdir -p "${DEB_DIR}/etc/battery-manager"
mkdir -p "${DEB_DIR}/usr/share/doc/${APP_NAME}"
mkdir -p "${DEB_DIR}/usr/share/battery-manager/docs"

# Copier le binaire
echo -e "${BLUE}Copie des fichiers...${NC}"
cp "${PROJECT_ROOT}/target/release/battery_manager" "${DEB_DIR}/usr/bin/battery-manager"
chmod 755 "${DEB_DIR}/usr/bin/battery-manager"
echo -e "${GREEN}✓${NC} Binaire copié"

# Copier l'icône
cp "${PROJECT_ROOT}/resources/icon.png" "${DEB_DIR}/usr/share/pixmaps/battery-manager.png"
chmod 644 "${DEB_DIR}/usr/share/pixmaps/battery-manager.png"
echo -e "${GREEN}✓${NC} Icône copiée"

# Copier le script de restauration
cp "${PROJECT_ROOT}/resources/battery-manager-restore.sh" "${DEB_DIR}/usr/bin/battery-manager-restore"
chmod 755 "${DEB_DIR}/usr/bin/battery-manager-restore"
echo -e "${GREEN}✓${NC} Script de restauration copié"

# Copier le fichier .desktop
cp "${PROJECT_ROOT}/resources/battery-manager.desktop" "${DEB_DIR}/usr/share/applications/"
chmod 644 "${DEB_DIR}/usr/share/applications/battery-manager.desktop"
echo -e "${GREEN}✓${NC} Fichier .desktop copié"

# Copier le service systemd
cp "${PROJECT_ROOT}/resources/battery-manager.service" "${DEB_DIR}/lib/systemd/system/"
chmod 644 "${DEB_DIR}/lib/systemd/system/battery-manager.service"
echo -e "${GREEN}✓${NC} Service systemd copié"

# Copier la documentation offline (HTML/CSS + icône)
echo -e "${BLUE}Copie de la documentation offline...${NC}"
cp "${PROJECT_ROOT}/docs/README.html" "${DEB_DIR}/usr/share/battery-manager/docs/README.html"
cp "${PROJECT_ROOT}/docs/REFERENCES.html" "${DEB_DIR}/usr/share/battery-manager/docs/REFERENCES.html"
cp "${PROJECT_ROOT}/docs/style.css" "${DEB_DIR}/usr/share/battery-manager/docs/style.css"
cp "${PROJECT_ROOT}/docs/icon.png" "${DEB_DIR}/usr/share/battery-manager/docs/icon.png"
chmod 644 "${DEB_DIR}/usr/share/battery-manager/docs/README.html"
chmod 644 "${DEB_DIR}/usr/share/battery-manager/docs/REFERENCES.html"
chmod 644 "${DEB_DIR}/usr/share/battery-manager/docs/style.css"
chmod 644 "${DEB_DIR}/usr/share/battery-manager/docs/icon.png"
echo -e "${GREEN}✓${NC} Documentation copiée"

# Créer le répertoire de configuration
mkdir -p "${DEB_DIR}/etc/battery-manager"
echo -e "${GREEN}✓${NC} Répertoire de configuration créé"

# Créer la documentation
cat > "${DEB_DIR}/usr/share/doc/${APP_NAME}/copyright" << 'EOF'
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: battery-manager
Source: https://github.com/yourusername/battery-manager

Files: *
Copyright: 2026 Battery Manager Team
License: MIT

License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a
 copy of this software and associated documentation files (the "Software"),
 to deal in the Software without restriction, including without limitation
 the rights to use, copy, modify, merge, publish, distribute, sublicense,
 and/or sell copies of the Software, and to permit persons to whom the
 Software is furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included
 in all copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
EOF

# Copier le README si disponible
if [ -f "../README.md" ]; then
    cp ../README.md "${DEB_DIR}/usr/share/doc/${APP_NAME}/README.md"
    gzip -9 -n "${DEB_DIR}/usr/share/doc/${APP_NAME}/README.md"
fi

# Créer le changelog
cat > "${DEB_DIR}/usr/share/doc/${APP_NAME}/changelog.Debian" << EOF
${APP_NAME} (${VERSION}) unstable; urgency=medium

  * Version initiale
  * Interface GTK4 pour la gestion de la batterie
  * Configuration des seuils de charge
  * Service systemd de restauration
  * Affichage en temps réel des informations

 -- ${MAINTAINER}  $(date -R)
EOF
gzip -9 -n "${DEB_DIR}/usr/share/doc/${APP_NAME}/changelog.Debian"

echo -e "${GREEN}✓${NC} Documentation créée"

# Calculer les tailles
INSTALLED_SIZE=$(du -sk "${DEB_DIR}" | cut -f1)
echo "Installed-Size: ${INSTALLED_SIZE}" >> "${DEB_DIR}/DEBIAN/control"

# Construire le package .deb
echo -e "\n${BLUE}Construction du package .deb...${NC}"
dpkg-deb --build --root-owner-group "${DEB_DIR}"

# Déplacer le .deb dans target/
mv "${DEB_DIR}.deb" "../target/${PACKAGE_NAME}.deb"

echo -e "\n${GREEN}═══════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Package créé avec succès !${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}\n"
echo -e "Fichier : ${BLUE}target/${PACKAGE_NAME}.deb${NC}"
echo -e "Taille  : $(du -h "../target/${PACKAGE_NAME}.deb" | cut -f1)\n"

echo -e "${YELLOW}Pour installer :${NC}"
echo -e "  sudo dpkg -i target/${PACKAGE_NAME}.deb"
echo -e "  sudo apt-get install -f  ${GREEN}# Si des dépendances manquent${NC}\n"

echo -e "${YELLOW}Pour désinstaller :${NC}"
echo -e "  sudo apt remove ${APP_NAME}          ${GREEN}# Garde la configuration${NC}"
echo -e "  sudo apt purge ${APP_NAME}           ${GREEN}# Supprime tout${NC}\n"

echo -e "${YELLOW}Pour vérifier :${NC}"
echo -e "  dpkg -c target/${PACKAGE_NAME}.deb   ${GREEN}# Voir le contenu${NC}"
echo -e "  dpkg -I target/${PACKAGE_NAME}.deb   ${GREEN}# Voir les infos${NC}\n"
