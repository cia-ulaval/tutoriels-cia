#![allow(dead_code)]

use std::{
    collections::HashMap,
    process::exit,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    time::Duration,
};

use rand::{Rng, seq::IndexedRandom};

use crate::boisson::{Boisson, CommandeBoisson};

#[derive(Debug, Clone)]
pub struct Requete {
    pub client: String,
    pub commande: CommandeBoisson,
    pub argent: f32,
}

#[derive(Debug)]
pub enum RefuseRaison {
    ManqueArgent,
    CommandeImpossible,
}

#[derive(Debug)]
pub struct Reponse {
    pub client: String,
    pub boisson: Boisson,
    pub monnaie: f32,
}

pub enum GameMode {
    Facil,
    Difficil,
}

struct GameState {
    requete_send: Sender<Requete>,
    reponse_recv: Receiver<Reponse>,
    mode: GameMode,
    /// Callbacks for each request sent by a client
    callbacks: Arc<Mutex<HashMap<String, Sender<Reponse>>>>,
}

const NOMS: &'static [&'static str] = &[
    "Aimé",
    "Anatole",
    "Clovis",
    "Godefroy",
    "Octave",
    "Léandre",
    "Théophile",
    "Armand",
    "Firmin",
    "Barthélemy",
];
const COMMANDES: &'static [CommandeBoisson] = &[
    CommandeBoisson::Expresso,
    CommandeBoisson::CafeAllonge,
    CommandeBoisson::CafeAuLait,
];

const REPOSE_TIMEOUT: Duration = Duration::from_secs(5);

static SCORE: Mutex<f32> = Mutex::new(0.0);

pub fn init_game(mode: GameMode) -> (Sender<Reponse>, Receiver<Requete>) {
    let (requete_send, requete_recv) = mpsc::channel();

    let (reponse_send, reponse_recv) = mpsc::channel();

    let state = GameState {
        requete_send,
        reponse_recv,
        mode,
        callbacks: Arc::new(Mutex::new(HashMap::<String, Sender<Reponse>>::new())),
    };

    std::thread::spawn(move || {
        run_game(state);
    });

    (reponse_send, requete_recv)
}

fn run_game(state: GameState) -> ! {
    // On lance un thread pour relayer les réponses aux theads des clients
    std::thread::spawn({
        let callbacks = state.callbacks.clone();
        move || {
            loop {
                let reponse = state.reponse_recv.recv().unwrap();
                let mut callbacks = callbacks.lock().unwrap();
                if let Some(callback) = callbacks.remove(&reponse.client) {
                    callback.send(reponse).unwrap();
                }
            }
        }
    });

    let mut rng = rand::rng();
    loop {
        // Spawn un nouveau client
        let callbacks = state.callbacks.clone();
        let requete_send = state.requete_send.clone();
        std::thread::spawn(move || {
            run_client(requete_send, callbacks);
        });

        let sleep_secs = match state.mode {
            GameMode::Difficil => rng.random_range(0.0..1.0),
            GameMode::Facil => rng.random_range(2.0..3.0),
        };
        std::thread::sleep(Duration::from_secs_f64(sleep_secs));
    }
}

fn run_client(
    requete_send: Sender<Requete>,
    callbacks: Arc<Mutex<HashMap<String, Sender<Reponse>>>>,
) {
    let mut rng = rand::rng();

    let (reponse_send, reponse_recv) = mpsc::channel();
    let mut nom = NOMS.choose(&mut rng).unwrap().to_string();
    {
        let mut callbacks = callbacks.lock().unwrap();
        while callbacks.contains_key(&nom) {
            let letter = (b'A' + rng.random_range(0..26)) as char;
            nom = format!("{} {}.", nom, &letter.to_string());
        }
        callbacks.insert(nom.clone(), reponse_send);
    }
    println!("[{nom}] *Arrive dans le café.*");

    let sleep_secs = rng.random_range(0.0..1.0);
    std::thread::sleep(Duration::from_secs_f64(sleep_secs));

    let commande = COMMANDES.choose(&mut rng).unwrap().clone();
    let extra = rng.random_range(0..10) as f32 * 0.25;
    let argent = commande.prix() + extra;

    let requete = Requete {
        client: nom.to_string(),
        commande: commande.clone(),
        argent,
    };

    // envoyer la requete
    requete_send.send(requete.clone()).unwrap();
    println!("[{}] *Passe sa commande: {:?}*", &nom, &commande);

    let sleep_secs = rng.random_range(0.0..3.0);
    std::thread::sleep(Duration::from_secs_f64(sleep_secs));

    // recevoir réponse
    if let Ok(reponse) = reponse_recv.recv_timeout(REPOSE_TIMEOUT) {
        let monnaie_attendue = requete.argent - requete.commande.prix();
        if monnaie_attendue != reponse.monnaie {
            println!("[{}] Eh, tu m'as pas donné le bon montant !", &nom);
            ajouter_au_score(-1.0);
        } else {
            let (score, commentaire) = reponse.boisson.evaluer_qualite(requete.commande);
            println!("[{}] {}", &nom, commentaire);
            ajouter_au_score(score);
        }
    } else {
        println!("[{nom}]: Ahh, j'ai trop attendu. Je m'en vais!");
        ajouter_au_score(-1.0);
        let mut callbacks = callbacks.lock().unwrap();
        callbacks.remove(&nom);
    }
}

fn ajouter_au_score(points: f32) {
    let mut score = SCORE.lock().unwrap();
    *score += points;
    if points > 0.0 {
        println!("SCORE = {score} (+{points})");
    } else {
        println!("SCORE = {score} (-{})", -points);
    }
    if *score < 0.0 {
        println!("SCORE NÉGATIF: GAME OVER");
        exit(0);
    } else if *score > 50.0 {
        println!("BRAVO : Vous avez gagné!");
        exit(0);
    }
}
