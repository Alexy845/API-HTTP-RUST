use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct RequestHeaders {
    method: String,
    path: String,
    user_agent: Option<String>,
    accept: Option<String>,
    host: Option<String>,
    connection: Option<String>,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);

        // Si la requête est un GET /ping, on retourne les headers
        if request.starts_with("GET /ping ") {
            let headers = parse_headers(&request);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                serde_json::to_string(&headers).unwrap()
            );
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

// Fonction pour extraire les headers de la requête
fn parse_headers(request: &str) -> RequestHeaders {
    // Diviser la requête en lignes et extraire les en-têtes
    let mut headers = RequestHeaders {
        method: "GET".to_string(),
        path: "/ping".to_string(),
        user_agent: None,
        accept: None,
        host: None,
        connection: None,
    };

    // Split de la requête par lignes
    let lines: Vec<&str> = request.lines().collect();
    
    for line in lines.iter() {
        if line.starts_with("GET") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                headers.path = parts[1].to_string();
            }
        }
        if line.starts_with("User-Agent:") {
            headers.user_agent = Some(line.split_at(12).1.trim().to_string());
        } else if line.starts_with("Accept:") {
            headers.accept = Some(line.split_at(7).1.trim().to_string());
        } else if line.starts_with("Host:") {
            headers.host = Some(line.split_at(5).1.trim().to_string());
        } else if line.starts_with("Connection:") {
            headers.connection = Some(line.split_at(11).1.trim().to_string());
        }
    }

    headers
}

fn main() {
    // Récupérer le port via la variable d'environnement ou utiliser 8080 par défaut
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Impossible de lier le port");

    println!("Serveur en écoute sur 127.0.0.1:{}", port);

    // Accepter les connexions entrantes et traiter chaque client
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}
