# Utilisation d'une image légère comme image finale
FROM debian:bullseye-slim

# Répertoire de travail
WORKDIR /app

# Définit le nom de l'exécutable (qui devrait être dans un dossier en dehors du contexte)
ARG EXECUTABLE_NAME=wrapper_gen

# Architecture de la plateforme cible
ARG TARGETPLATFORM
ARG TARGETARCH
ARG BINARY_DIR


# Copie l'exécutable en fonction de l'architecture cible
# Veuillez remplacer `/path/to/external/binaries` par le chemin réel vers les fichiers
COPY ./target/$TARGETPLATFORM/$EXECUTABLE_NAME /app/$EXECUTABLE_NAME
# Définition du point d'entrée
ENTRYPOINT ["/app/wrapper_gen"]