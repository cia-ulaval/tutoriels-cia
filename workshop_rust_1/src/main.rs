mod boisson;
mod jeu;

use std::thread::sleep;
use std::time::Duration;

use crate::boisson::{Boisson, CommandeBoisson, ConteneurLait, MachineExpresso};
use crate::jeu::{GameMode, Reponse, init_game};

fn main() {
    // Initialisation de l'équipement du café, ne pas changer ces lignes:
    let mut machine_espresso = MachineExpresso::<4>::new();
    let mut conteneur_lait = ConteneurLait {};

    // Initialisation du jeu : les clients vont commencer à arriver !
    let (reponse_send, requete_recv) = init_game(GameMode::Difficil);

    loop {
        // faites votre algorithme de cafe ici !
    }
}
