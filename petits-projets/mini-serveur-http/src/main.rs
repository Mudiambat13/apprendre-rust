//! `cargo run -p mini-serveur-http -- [port] [nb_workers]`  (défaut : 7878, 4 workers)
//!
//! Puis ouvrez <http://127.0.0.1:7878/> dans un navigateur, ou : `curl http://127.0.0.1:7878/somme?a=2&b=40`
//!
//! Le serveur n'écoute que sur `127.0.0.1` (votre machine) : ne l'exposez pas sur Internet tel quel.

use std::net::TcpListener;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let port: u16 = args.first().and_then(|p| p.parse().ok()).unwrap_or(7878);
    let workers: usize = args.get(1).and_then(|p| p.parse().ok()).unwrap_or(4);

    let ecouteur = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("impossible d'écouter sur le port {port} : {e}");
            return ExitCode::FAILURE;
        }
    };
    println!("🦀 serveur sur http://127.0.0.1:{port}/ ({workers} workers) — Ctrl-C pour arrêter");

    match mini_serveur_http::servir(ecouteur, workers, None) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
