#!/bin/bash

# Chemin local des fichiers de configuration
CONFIG_DIR="./tests/res"

# Variable de configuration pour le fichier
ENTRY_CONFIG_FILE="/config/test.json"

# Vérifiez que le répertoire existe
if [ ! -d "$CONFIG_DIR" ]; then
  echo "Configuration directory $CONFIG_DIR does not exist."
  exit 1
fi

# Lancer les conteneurs avec le répertoire de configuration monté
docker run --rm -v "$CONFIG_DIR/main:/config" -e CONFIG_FILE="$ENTRY_CONFIG_FILE" kerneltzo/command_runner:main-dev || exit 1
docker run --rm -v "$CONFIG_DIR/cfssl:/config" -e CONFIG_FILE="$ENTRY_CONFIG_FILE" kerneltzo/command_runner:cfssl-dev || exit 1

echo "All tests passed"
exit 0
