//! # Un mini serveur HTTP/1.1
//!
//! Réutilise le [`ThreadPool`](thread_pool::ThreadPool) du projet `thread-pool` : chaque
//! connexion acceptée est confiée à un worker.
//!
//! ## Ce que ce projet illustre
//!
//! - **Séparation des responsabilités** : analyse (`Requete::analyser`), logique (`router`),
//!   sérialisation (`Reponse::vers_octets`) et entrées/sorties réseau (`servir`) sont séparées,
//!   donc les trois premières sont testables **sans réseau**.
//! - **Programmation défensive** : taille des lignes limitée, délai de lecture, nombre d'en-têtes
//!   borné, *échappement HTML* des données utilisateur (sinon : faille XSS), décodage des `%xx`.
//! - **Tests d'intégration réseau** sur `127.0.0.1:0` (le système choisit un port libre).
//!
//! Limites assumées (c'est un exercice) : pas de *keep-alive*, pas de corps de requête, pas de TLS.
//! Pour un vrai serveur : `axum`, `actix-web`, `hyper`.

use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use thread_pool::ThreadPool;

const LIGNE_MAX: u64 = 8 * 1024;
const EN_TETES_MAX: usize = 64;

// ───────────────────────────── Requête ─────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum ErreurHttp {
    LigneVide,
    LigneInvalide,
    VersionNonSupportee,
    TropLong,
    EnTeteInvalide,
}

impl fmt::Display for ErreurHttp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = match self {
            ErreurHttp::LigneVide => "requête vide",
            ErreurHttp::LigneInvalide => "ligne de requête invalide",
            ErreurHttp::VersionNonSupportee => "version HTTP non supportée",
            ErreurHttp::TropLong => "requête trop volumineuse",
            ErreurHttp::EnTeteInvalide => "en-tête invalide",
        };
        f.write_str(m)
    }
}

impl std::error::Error for ErreurHttp {}

#[derive(Debug, PartialEq, Eq)]
pub struct Requete {
    pub methode: String,
    pub chemin: String,
    pub requete: Option<String>, // la partie après « ? », non décodée
    pub en_tetes: Vec<(String, String)>,
}

impl Requete {
    /// Analyse une requête complète (ligne de requête + en-têtes), terminée ou non par une ligne vide.
    pub fn analyser(texte: &str) -> Result<Requete, ErreurHttp> {
        let mut lignes = texte.lines();
        let premiere = lignes
            .next()
            .filter(|l| !l.trim().is_empty())
            .ok_or(ErreurHttp::LigneVide)?;

        let mut morceaux = premiere.split(' ');
        let (Some(methode), Some(cible), Some(version), None) = (
            morceaux.next(),
            morceaux.next(),
            morceaux.next(),
            morceaux.next(),
        ) else {
            return Err(ErreurHttp::LigneInvalide);
        };
        if version != "HTTP/1.1" && version != "HTTP/1.0" {
            return Err(ErreurHttp::VersionNonSupportee);
        }
        if methode.is_empty()
            || !methode.bytes().all(|b| b.is_ascii_uppercase())
            || !cible.starts_with('/')
        {
            return Err(ErreurHttp::LigneInvalide);
        }

        let (chemin, requete) = match cible.split_once('?') {
            Some((c, q)) => (c.to_string(), Some(q.to_string())),
            None => (cible.to_string(), None),
        };

        let mut en_tetes = Vec::new();
        for ligne in lignes.take_while(|l| !l.is_empty()) {
            if en_tetes.len() >= EN_TETES_MAX {
                return Err(ErreurHttp::TropLong);
            }
            let (nom, valeur) = ligne.split_once(':').ok_or(ErreurHttp::EnTeteInvalide)?;
            en_tetes.push((nom.trim().to_ascii_lowercase(), valeur.trim().to_string()));
        }

        Ok(Requete {
            methode: methode.to_string(),
            chemin,
            requete,
            en_tetes,
        })
    }

    pub fn en_tete(&self, nom: &str) -> Option<&str> {
        let nom = nom.to_ascii_lowercase();
        self.en_tetes
            .iter()
            .find(|(n, _)| *n == nom)
            .map(|(_, v)| v.as_str())
    }

    /// Valeur d'un paramètre de requête (`?a=1&b=2`), décodée.
    pub fn parametre(&self, nom: &str) -> Option<String> {
        self.requete.as_deref()?.split('&').find_map(|paire| {
            let (k, v) = paire.split_once('=').unwrap_or((paire, ""));
            (decoder_url(k) == nom).then(|| decoder_url(v))
        })
    }
}

// ───────────────────────────── Réponse ─────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub struct Reponse {
    pub code: u16,
    pub type_contenu: &'static str,
    pub corps: Vec<u8>,
}

fn phrase(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Internal Server Error",
    }
}

impl Reponse {
    pub fn html(code: u16, corps: impl Into<String>) -> Self {
        Reponse {
            code,
            type_contenu: "text/html; charset=utf-8",
            corps: corps.into().into_bytes(),
        }
    }

    pub fn texte(code: u16, corps: impl Into<String>) -> Self {
        Reponse {
            code,
            type_contenu: "text/plain; charset=utf-8",
            corps: corps.into().into_bytes(),
        }
    }

    pub fn vers_octets(&self) -> Vec<u8> {
        let entete = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
            self.code,
            phrase(self.code),
            self.type_contenu,
            self.corps.len()
        );
        let mut octets = entete.into_bytes();
        octets.extend_from_slice(&self.corps);
        octets
    }
}

// ───────────────────────────── Utilitaires ─────────────────────────────

/// Échappe les caractères spéciaux HTML : **indispensable** avant d'insérer du texte utilisateur dans une page.
pub fn echapper_html(texte: &str) -> String {
    let mut s = String::with_capacity(texte.len());
    for c in texte.chars() {
        match c {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '"' => s.push_str("&quot;"),
            '\'' => s.push_str("&#39;"),
            c => s.push(c),
        }
    }
    s
}

/// Décode les `%xx` et les `+` (espace) d'une URL. Les séquences invalides sont conservées telles quelles.
pub fn decoder_url(texte: &str) -> String {
    let octets = texte.as_bytes();
    let mut sortie = Vec::with_capacity(octets.len());
    let mut i = 0;
    while i < octets.len() {
        match octets[i] {
            b'+' => sortie.push(b' '),
            b'%' if octets
                .get(i + 1..i + 3)
                .is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit)) =>
            {
                let hex = std::str::from_utf8(&octets[i + 1..i + 3]).unwrap_or("00");
                sortie.push(u8::from_str_radix(hex, 16).unwrap_or(b'?'));
                i += 2;
            }
            b => sortie.push(b),
        }
        i += 1;
    }
    String::from_utf8_lossy(&sortie).into_owned()
}

// ───────────────────────────── Routage ─────────────────────────────

/// Toute la logique « métier », sans aucun accès réseau.
pub fn router(requete: &Requete) -> Reponse {
    if requete.methode != "GET" {
        return Reponse::texte(405, "méthode non autorisée : seul GET est supporté");
    }
    match requete.chemin.as_str() {
        "/" => Reponse::html(
            200,
            "<!doctype html><meta charset=utf-8><title>Mini serveur Rust</title>\
             <h1>🦀 Mini serveur HTTP</h1><ul>\
             <li><a href=\"/bonjour/Alice\">/bonjour/&lt;nom&gt;</a></li>\
             <li><a href=\"/heure\">/heure</a></li>\
             <li><a href=\"/echo?texte=salut+le+monde\">/echo?texte=…</a></li>\
             <li><a href=\"/somme?a=2&amp;b=40\">/somme?a=…&amp;b=…</a></li></ul>",
        ),
        "/heure" => {
            let secondes = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| d.as_secs());
            Reponse::texte(200, format!("secondes depuis 1970 : {secondes}"))
        }
        "/echo" => match requete.parametre("texte") {
            // Texte brut + `nosniff` : pas d'interprétation HTML par le navigateur.
            Some(t) => Reponse::texte(200, t),
            None => Reponse::texte(400, "paramètre « texte » manquant"),
        },
        "/somme" => match (
            requete.parametre("a").and_then(|a| a.parse::<i64>().ok()),
            requete.parametre("b").and_then(|b| b.parse::<i64>().ok()),
        ) {
            (Some(a), Some(b)) => match a.checked_add(b) {
                Some(s) => Reponse::texte(200, s.to_string()),
                None => Reponse::texte(400, "dépassement de capacité"),
            },
            _ => Reponse::texte(400, "paramètres « a » et « b » (entiers) requis"),
        },
        chemin => match chemin.strip_prefix("/bonjour/") {
            Some(nom) if !nom.is_empty() && !nom.contains('/') => {
                let nom = decoder_url(nom);
                Reponse::html(
                    200,
                    format!(
                        "<!doctype html><meta charset=utf-8><h1>Bonjour {} !</h1>",
                        echapper_html(&nom)
                    ),
                )
            }
            _ => Reponse::html(
                404,
                format!(
                    "<h1>404</h1><p>« {} » n'existe pas.</p>",
                    echapper_html(chemin)
                ),
            ),
        },
    }
}

// ───────────────────────────── Réseau ─────────────────────────────

/// Lit jusqu'à la ligne vide qui termine les en-têtes, avec des limites strictes.
fn lire_requete_brute(flux: &TcpStream) -> Result<String, ErreurHttp> {
    let mut lecteur = BufReader::new(flux);
    let mut brut = String::new();
    for _ in 0..=EN_TETES_MAX + 1 {
        let mut ligne = String::new();
        // `take` borne la taille d'une ligne : sans cela, un client pourrait envoyer des Go sans « \n ».
        let n = (&mut lecteur)
            .take(LIGNE_MAX)
            .read_line(&mut ligne)
            .map_err(|_| ErreurHttp::LigneInvalide)?;
        if n == 0 {
            break;
        }
        if n as u64 == LIGNE_MAX && !ligne.ends_with('\n') {
            return Err(ErreurHttp::TropLong);
        }
        let fin = ligne.trim_end_matches(['\r', '\n']).is_empty();
        brut.push_str(&ligne);
        if fin {
            return Ok(brut);
        }
    }
    Err(ErreurHttp::TropLong)
}

fn traiter_connexion(mut flux: TcpStream) -> io::Result<()> {
    flux.set_read_timeout(Some(Duration::from_secs(5)))?;
    flux.set_write_timeout(Some(Duration::from_secs(5)))?;

    let (reponse, refusee) =
        match lire_requete_brute(&flux).and_then(|brut| Requete::analyser(&brut)) {
            Ok(requete) => (router(&requete), false),
            Err(e) => (Reponse::texte(400, format!("requête invalide : {e}")), true),
        };
    flux.write_all(&reponse.vers_octets())?;
    flux.flush()?;

    if refusee {
        // Fermer une connexion alors que le client envoie encore des octets provoque un « reset »
        // (RST) qui peut effacer notre réponse côté client. On clôt donc proprement l'écriture,
        // puis on vide — sans dépasser 64 Kio — ce que le client avait déjà envoyé.
        let _ = flux.shutdown(Shutdown::Write);
        let _ = io::copy(&mut (&flux).take(64 * 1024), &mut io::sink());
    }
    Ok(())
}

/// Accepte des connexions et les confie à un pool de `nb_workers` threads.
///
/// - `max_connexions = None` : tourne indéfiniment ;
/// - `Some(n)` : s'arrête après `n` connexions (pratique pour les tests), puis attend la fin
///   des tâches en cours (grâce au `Drop` du pool).
pub fn servir(
    ecouteur: TcpListener,
    nb_workers: usize,
    max_connexions: Option<usize>,
) -> io::Result<()> {
    let pool =
        ThreadPool::new(nb_workers).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut acceptees = 0usize;
    for flux in ecouteur.incoming() {
        match flux {
            Ok(flux) => {
                pool.execute(move || {
                    // Une erreur sur UNE connexion (client parti, délai dépassé) ne doit pas arrêter le serveur.
                    if let Err(e) = traiter_connexion(flux) {
                        eprintln!("connexion abandonnée : {e}");
                    }
                });
                acceptees += 1;
                if max_connexions.is_some_and(|m| acceptees >= m) {
                    break;
                }
            }
            Err(e) => eprintln!("échec d'acceptation : {e}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::thread;

    fn requete(texte: &str) -> Requete {
        Requete::analyser(texte).expect("requête valide")
    }

    fn get(chemin: &str) -> Reponse {
        router(&requete(&format!(
            "GET {chemin} HTTP/1.1\r\nHost: x\r\n\r\n"
        )))
    }

    fn corps(r: &Reponse) -> String {
        String::from_utf8_lossy(&r.corps).into_owned()
    }

    #[test]
    fn analyse_une_requete_complete() {
        let r = requete(
            "GET /echo?texte=a%20b&x=1 HTTP/1.1\r\nHost: exemple.org\r\nUser-Agent:  test \r\n\r\n",
        );
        assert_eq!(r.methode, "GET");
        assert_eq!(r.chemin, "/echo");
        assert_eq!(r.requete.as_deref(), Some("texte=a%20b&x=1"));
        assert_eq!(r.en_tete("HOST"), Some("exemple.org"));
        assert_eq!(r.en_tete("user-agent"), Some("test"));
        assert_eq!(r.parametre("texte").as_deref(), Some("a b"));
        assert_eq!(r.parametre("x").as_deref(), Some("1"));
        assert_eq!(r.parametre("absent"), None);
    }

    #[test]
    fn requetes_invalides() {
        assert_eq!(Requete::analyser(""), Err(ErreurHttp::LigneVide));
        assert_eq!(
            Requete::analyser("GET /\r\n"),
            Err(ErreurHttp::LigneInvalide)
        );
        assert_eq!(
            Requete::analyser("GET / HTTP/1.1 extra\r\n"),
            Err(ErreurHttp::LigneInvalide)
        );
        assert_eq!(
            Requete::analyser("get / HTTP/1.1\r\n"),
            Err(ErreurHttp::LigneInvalide)
        );
        assert_eq!(
            Requete::analyser("GET index HTTP/1.1\r\n"),
            Err(ErreurHttp::LigneInvalide)
        );
        assert_eq!(
            Requete::analyser("GET / HTTP/2\r\n"),
            Err(ErreurHttp::VersionNonSupportee)
        );
        assert_eq!(
            Requete::analyser("GET / HTTP/1.1\r\nsans-deux-points\r\n\r\n"),
            Err(ErreurHttp::EnTeteInvalide)
        );
        let trop = format!("GET / HTTP/1.1\r\n{}\r\n", "A: b\r\n".repeat(100));
        assert_eq!(Requete::analyser(&trop), Err(ErreurHttp::TropLong));
    }

    #[test]
    fn decodage_d_url() {
        assert_eq!(decoder_url("a%20b+c"), "a b c");
        assert_eq!(decoder_url("%C3%A9t%C3%A9"), "été");
        assert_eq!(decoder_url("100%"), "100%");
        assert_eq!(decoder_url("%zz%4"), "%zz%4");
        assert_eq!(decoder_url("%41"), "A");
    }

    #[test]
    fn echappement_html() {
        assert_eq!(
            echapper_html("<script>alert('x')</script> & \"q\""),
            "&lt;script&gt;alert(&#39;x&#39;)&lt;/script&gt; &amp; &quot;q&quot;"
        );
    }

    #[test]
    fn routes_de_base() {
        assert_eq!(get("/").code, 200);
        assert!(corps(&get("/")).contains("Mini serveur"));
        assert_eq!(corps(&get("/somme?a=2&b=40")), "42");
        assert_eq!(corps(&get("/echo?texte=salut+le+monde")), "salut le monde");
        assert_eq!(get("/echo").code, 400);
        assert_eq!(get("/somme?a=x&b=1").code, 400);
        assert_eq!(get("/somme?a=9223372036854775807&b=1").code, 400);
        assert!(corps(&get("/heure")).starts_with("secondes depuis 1970"));
        assert_eq!(get("/inexistant").code, 404);
    }

    #[test]
    fn la_methode_doit_etre_get() {
        let r = router(&requete("POST / HTTP/1.1\r\n\r\n"));
        assert_eq!(r.code, 405);
    }

    #[test]
    fn pas_de_faille_xss() {
        let r = get("/bonjour/%3Cscript%3Ealert(1)%3C%2Fscript%3E");
        assert_eq!(r.code, 200);
        assert!(!corps(&r).contains("<script>"), "{}", corps(&r));
        assert!(corps(&r).contains("&lt;script&gt;"));
        // Même protection sur la page 404 :
        let r = get("/%3Cimg%20src=x%3E");
        assert!(!corps(&r).contains("<img"), "{}", corps(&r));
    }

    #[test]
    fn bonjour_accepte_l_unicode() {
        assert!(corps(&get("/bonjour/Ren%C3%A9e")).contains("Bonjour Renée !"));
    }

    #[test]
    fn serialisation_de_la_reponse() {
        let octets = Reponse::texte(200, "hé").vers_octets();
        let texte = String::from_utf8(octets).unwrap();
        assert!(texte.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(texte.contains("Content-Length: 3\r\n")); // « é » = 2 octets
        assert!(texte.ends_with("\r\n\r\nhé"));
    }

    /// Démarre un vrai serveur sur un port libre, qui traitera exactement `n` connexions.
    fn lancer(n: usize) -> (SocketAddr, thread::JoinHandle<()>) {
        let ecouteur = TcpListener::bind("127.0.0.1:0").unwrap();
        let adresse = ecouteur.local_addr().unwrap();
        let fil = thread::spawn(move || servir(ecouteur, 2, Some(n)).unwrap());
        (adresse, fil)
    }

    fn requete_http(adresse: SocketAddr, brut: &[u8]) -> String {
        let mut flux = TcpStream::connect(adresse).unwrap();
        flux.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        flux.write_all(brut).unwrap();
        let mut reponse = Vec::new();
        flux.read_to_end(&mut reponse).unwrap();
        String::from_utf8_lossy(&reponse).into_owned()
    }

    #[test]
    fn serveur_de_bout_en_bout() {
        let (adresse, fil) = lancer(4);

        let r = requete_http(adresse, b"GET /somme?a=20&b=22 HTTP/1.1\r\nHost: x\r\n\r\n");
        assert!(r.starts_with("HTTP/1.1 200 OK"), "{r}");
        assert!(r.ends_with("42"), "{r}");

        let r = requete_http(adresse, b"GET /rien HTTP/1.1\r\n\r\n");
        assert!(r.starts_with("HTTP/1.1 404 Not Found"), "{r}");

        let r = requete_http(adresse, b"DELETE / HTTP/1.1\r\n\r\n");
        assert!(r.starts_with("HTTP/1.1 405"), "{r}");

        let r = requete_http(adresse, b"N'IMPORTE QUOI\r\n\r\n");
        assert!(r.starts_with("HTTP/1.1 400"), "{r}");

        fil.join().unwrap();
    }

    #[test]
    fn requetes_concurrentes() {
        let (adresse, fil) = lancer(8);
        let clients: Vec<_> = (0..8)
            .map(|i| {
                thread::spawn(move || {
                    requete_http(
                        adresse,
                        format!("GET /somme?a={i}&b=100 HTTP/1.1\r\n\r\n").as_bytes(),
                    )
                })
            })
            .collect();
        for (i, c) in clients.into_iter().enumerate() {
            let r = c.join().unwrap();
            assert!(r.ends_with(&(100 + i).to_string()), "{r}");
        }
        fil.join().unwrap();
    }

    #[test]
    fn une_ligne_geante_est_refusee() {
        let (adresse, fil) = lancer(1);
        let mut brut = b"GET /".to_vec();
        brut.extend(std::iter::repeat_n(b'a', 20_000));
        brut.extend_from_slice(b" HTTP/1.1\r\n\r\n");
        let r = requete_http(adresse, &brut);
        assert!(r.starts_with("HTTP/1.1 400"), "{}", &r[..r.len().min(80)]);
        fil.join().unwrap();
    }
}
