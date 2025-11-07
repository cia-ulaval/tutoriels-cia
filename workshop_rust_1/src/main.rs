mod boisson;
mod jeu;
// mod machine_espresso;

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

        let prochaine_requete = requete_recv.recv().unwrap();

        let mut boisson = Boisson::vide();
        match prochaine_requete.commande {
            CommandeBoisson::Expresso => {
                machine_espresso.commencer_expresso(0, boisson).unwrap();
                while !machine_espresso.est_termine(0).unwrap() {
                    sleep(Duration::from_millis(50));
                }
                boisson = machine_espresso.retirer_boisson(0).unwrap();
            }
            CommandeBoisson::CafeAllonge => {
                machine_espresso.commencer_expresso(0, boisson).unwrap();
                while !machine_espresso.est_termine(0).unwrap() {
                    sleep(Duration::from_millis(50));
                }
                boisson = machine_espresso.retirer_boisson(0).unwrap();
                machine_espresso.ajouter_eau_chaude(&mut boisson, 100.0);
            }
            CommandeBoisson::CafeAuLait => {
                machine_espresso.commencer_expresso(0, boisson).unwrap();
                while !machine_espresso.est_termine(0).unwrap() {
                    sleep(Duration::from_millis(50));
                }
                boisson = machine_espresso.retirer_boisson(0).unwrap();
                conteneur_lait.ajouter_lait(&mut boisson, 100.0);
            }
        }

        let monnaie = prochaine_requete.argent - prochaine_requete.commande.prix();
        let reponse = Reponse {
            client: prochaine_requete.client,
            boisson: boisson,
            monnaie: monnaie,
        };

        reponse_send.send(reponse).unwrap()
    }
}
