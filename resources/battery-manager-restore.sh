#!/bin/bash
# Script de restauration des seuils de charge au démarrage

CONFIG_DIR="/etc/battery-manager"

# Fonction pour appliquer les seuils
apply_threshold() {
    local battery="$1"
    local start="$2"
    local stop="$3"
    
    # Chemins possibles pour les seuils
    local start_paths=(
        "/sys/class/power_supply/$battery/charge_control_start_threshold"
        "/sys/class/power_supply/$battery/charge_start_threshold"
    )
    
    local stop_paths=(
        "/sys/class/power_supply/$battery/charge_control_end_threshold"
        "/sys/class/power_supply/$battery/charge_stop_threshold"
        "/sys/class/power_supply/$battery/charge_end_threshold"
    )
    
    # Appliquer le seuil de début (seulement si défini et fichier existe)
    if [[ -n "$start" ]]; then
        for path in "${start_paths[@]}"; do
            if [[ -f "$path" && -w "$path" ]]; then
                if echo "$start" > "$path" 2>/dev/null; then
                    echo "✓ Seuil de début appliqué: $start% ($battery)"
                    break
                fi
            fi
        done
    fi
    
    # Appliquer le seuil de fin
    local applied=false
    for path in "${stop_paths[@]}"; do
        if [[ -f "$path" && -w "$path" ]]; then
            if echo "$stop" > "$path" 2>/dev/null; then
                echo "✓ Seuil de fin appliqué: $stop% ($battery)"
                applied=true
                break
            fi
        fi
    done
    
    if [[ "$applied" == "false" ]]; then
        echo "⚠ Impossible d'appliquer le seuil de fin pour $battery"
    fi
}

# Restaurer les seuils pour chaque batterie configurée
if [[ -d "$CONFIG_DIR" ]]; then
    for config_file in "$CONFIG_DIR"/*.conf; do
        if [[ -f "$config_file" ]]; then
            battery=$(basename "$config_file" .conf)
            
            # Lire la configuration
            START_THRESHOLD=""
            STOP_THRESHOLD=""
            source "$config_file"
            
            if [[ -n "$STOP_THRESHOLD" ]]; then
                echo "Restauration des seuils pour $battery..."
                apply_threshold "$battery" "$START_THRESHOLD" "$STOP_THRESHOLD"
            else
                echo "⚠ Configuration incomplète pour $battery (STOP_THRESHOLD manquant)"
            fi
        fi
    done
else
    echo "Aucune configuration trouvée dans $CONFIG_DIR"
fi
