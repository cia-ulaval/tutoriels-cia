#![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender},
    },
    time::Duration,
};

use rand::{Rng, seq::IndexedRandom};

use crate::boisson::{Boisson, CommandeBoisson};

#[derive(Debug, Clone)]
pub struct Requette {
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
pub enum Reponse {
    Servie {
        client: String,
        boisson: Boisson,
        monnaie: f32,
    },
    Refuse {
        client: String,
        raison: RefuseRaison,
    },
}

impl Reponse {
    fn get_client(&self) -> String {
        match self {
            Reponse::Servie {
                client,
                boisson: _,
                monnaie: _,
            } => client.clone(),
            Reponse::Refuse { client, raison: _ } => client.clone(),
        }
    }
}

pub fn init_clients() -> (SyncSender<Reponse>, Receiver<Requette>) {
    let (requette_send, requette_recv) = mpsc::sync_channel(32);
    let (reponse_send, reponse_recv) = mpsc::sync_channel(32);

    std::thread::spawn(move || {
        run_clients(requette_send, reponse_recv);
    });

    (reponse_send, requette_recv)
}

fn run_clients(requette_send: SyncSender<Requette>, reponse_recv: Receiver<Reponse>) -> ! {
    // store incoming responses
    let reponses_store = Arc::new(Mutex::new(HashMap::<String, Reponse>::new()));

    std::thread::spawn({
        let reponses_store = reponses_store.clone();
        move || {
            loop {
                let reponse = reponse_recv.recv().unwrap();
                let mut reponses_store = reponses_store.lock().unwrap();
                reponses_store.insert(reponse.get_client(), reponse);
                // notify clients maybe
            }
        }
    });

    let mut rng = rand::rng();
    loop {
        // Spawn un nouveau client
        let reponses_store = reponses_store.clone();
        let requette_send = requette_send.clone();
        std::thread::spawn(move || {
            run_client(requette_send, reponses_store);
        });

        let sleep_secs = rng.random_range(4.0..5.0);
        std::thread::sleep(Duration::from_secs_f64(sleep_secs));
    }
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
    CommandeBoisson::Espresso,
    CommandeBoisson::CafeAllonge,
    CommandeBoisson::CafeLatte,
];

static SCORE: Mutex<f32> = Mutex::new(0.0);

fn run_client(
    requette_send: SyncSender<Requette>,
    reponses_store: Arc<Mutex<HashMap<String, Reponse>>>,
) {
    let mut rng = rand::rng();

    let letter = (b'A' + rng.random_range(0..26)) as char;
    let mut nom = NOMS.choose(&mut rng).unwrap().to_string();
    nom = format!("{} {}.", nom, &letter.to_string());
    println!("[{nom}] *Arrive dans le café.*");

    let sleep_secs = rng.random_range(0.0..1.0);
    std::thread::sleep(Duration::from_secs_f64(sleep_secs));

    let commande = COMMANDES.choose(&mut rng).unwrap().clone();
    let argent = commande.prix() + rng.random_range(0.0..3.0);

    let requette = Requette {
        client: nom.to_string(),
        commande: commande.clone(),
        argent,
    };

    // envoyer la requette
    requette_send.send(requette.clone()).unwrap();
    println!("[{}] *Passe sa commande: {:?}*", &nom, &commande);

    let sleep_secs = rng.random_range(0.0..3.0);
    std::thread::sleep(Duration::from_secs_f64(sleep_secs));

    // recevoir réponse
    if let Some(reponse) = recevoir_reponse(&nom, reponses_store) {
        match reponse {
            Reponse::Servie {
                client: _,
                boisson,
                monnaie,
            } => {
                let monnaie_attendue = requette.argent - requette.commande.prix();
                if monnaie_attendue != monnaie {
                    println!("[{}] Eh, tu m'as pas donné le bon montant !", &nom);
                    ajouter_au_score(-1.0);
                } else {
                    let (score, commentaire) = boisson.evaluer_qualite(requette.commande);
                    println!("[{}] {}", &nom, commentaire);
                    ajouter_au_score(score);
                }
            }
            Reponse::Refuse {
                client: _,
                raison: _,
            } => todo!(),
        }
    } else {
        println!("[{nom}]: Ahh, j'ai trop attendu. Je m'en vais!");
        ajouter_au_score(-1.0);
    }
}

fn recevoir_reponse(
    nom: &String,
    reponses_store: Arc<Mutex<HashMap<String, Reponse>>>,
) -> Option<Reponse> {
    const MAX_ATTENTE: i32 = 10; // en secondes
    for _ in 0..MAX_ATTENTE {
        std::thread::sleep(Duration::from_secs(1));
        let mut reponses_store = reponses_store.lock().unwrap();
        if let Some(reponse) = reponses_store.remove(nom) {
            return Some(reponse);
        }
    }
    return None;
}

fn ajouter_au_score(points: f32) {
    let mut score = SCORE.lock().unwrap();
    *score += points;
    println!("SCORE = {score} (+{points})");
}
