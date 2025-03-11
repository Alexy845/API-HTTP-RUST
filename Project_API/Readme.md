# API Rust - HTTP

## Description

Ce projet est une API en **Rust** qui implémente un serveur HTTP basique. L'API expose une route `GET /ping` qui retourne les en-têtes de la requête sous forme de JSON. Toutes les autres routes renvoient une réponse 404 avec un corps vide.

## Prérequis

Avant de commencer, assurez-vous d'avoir installé **Rust** sur votre machine. Vous pouvez télécharger et installer Rust à partir de [ici](https://www.rust-lang.org/).

### Vérifiez que Rust est installé :

```bash
rustc --version
cargo --version
```

Clonez le projet sur votre machine locale en utilisant la commande suivante :

```bash
git clone https://github.com/Alexy845/API-HTTP-RUST.git
cd .\Project_API\
```

## Installation
Aucune dépendance externe n'est nécessaire au-delà de Rust. Le projet est déjà configuré pour être utilisé avec Cargo, l'outil de gestion de projet Rust.

### Démarrer le serveur
Configurer le port (optionnel)
Le serveur utilise la variable d'environnement PING_LISTEN_PORT pour déterminer le port d'écoute. Si cette variable n'est pas définie, le serveur écoutera par défaut sur le port 8080.

Sous Linux/Mac :

```bash
export PING_LISTEN_PORT=8081
```

Sous Windows (PowerShell) :

```bash
$env:PING_LISTEN_PORT="8081"
```

Une fois que vous avez cloné le projet et éventuellement configuré le port, vous pouvez démarrer le serveur avec la commande suivante :

```bash
cargo run
```

Le serveur démarrera et écoutera sur 127.0.0.1:8080 ou le port que vous avez défini dans la variable d'environnement.

## Fonctionnalités
GET /ping : Retourne les en-têtes de la requête sous forme de JSON.
Autres routes : Renvoient une erreur 404 avec un corps vide.
Exemple d'utilisation
Requête GET /ping

Pour tester l'API, vous pouvez utiliser curl ou n'importe quel autre client HTTP.

```bash
curl http://127.0.0.1:8080/ping
```

La réponse sera un JSON contenant les en-têtes de la requête envoyée :

```json
{
  "method": "GET",
  "path": "/ping",
  "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 OPR/116.0.0.0 (Edition std-1)",
  "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
  "host": "127.0.0.1:8080",
  "connection": "keep-alive"
}
```

Si vous testez une autre route (par exemple /unknown), le serveur renverra une erreur 404 Not Found avec le message suivant :

```txt
HTTP/1.1 404 Not Found
```

## Technologies utilisées
- Rust
- TcpListener pour gérer les requêtes HTTP
- Serde pour la sérialisation des en-têtes de requêtes en JSON
- Serde_JSON pour la conversion des en-têtes en JSON








