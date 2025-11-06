mod boisson;
mod jeu;
// mod machine_espresso;

use std::thread::sleep;
use std::time::Duration;

use crate::boisson::{Boisson, ConteneurLait, CommandeBoisson, MachineEspresso};
use crate::jeu::{Reponse, init_clients};

fn main() {
    // Initialisation du café, ne pas changer ces lignes:
    let mut machine_espresso = MachineEspresso::<4>::new();
    let mut conteneur_lait = ConteneurLait {};
    // Initialisation des clients
    let (reponse_send, requette_recv) = init_clients();

    loop {
        // faites votre algorithme de cafe ici !
        // let prochaine_requette = requette_recv.recv().unwrap();

        //reponse_send.send(exemple_reponse).unwrap()

        // SOLUTION
        let prochaine_requette = requette_recv.recv().unwrap();

        let mut boisson = Boisson::vide();
        match prochaine_requette.commande {
            CommandeBoisson::Espresso => {
                machine_espresso.commencer_espresso(0, boisson).unwrap();
                while !machine_espresso.est_termine(0).unwrap() {
                    sleep(Duration::from_millis(50));
                }
                boisson = machine_espresso.retirer_boisson(0).unwrap();
            },
            CommandeBoisson::CafeAllonge => {
                machine_espresso.commencer_espresso(0, boisson).unwrap();
                while !machine_espresso.est_termine(0).unwrap() {
                    sleep(Duration::from_millis(50));
                }
                boisson = machine_espresso.retirer_boisson(0).unwrap();
                machine_espresso.ajouter_eau_chaude(&mut boisson, 100.0);
            },
            CommandeBoisson::CafeLatte => {
                machine_espresso.commencer_espresso(0, boisson).unwrap();
                while !machine_espresso.est_termine(0).unwrap() {
                    sleep(Duration::from_millis(50));
                }
                boisson = machine_espresso.retirer_boisson(0).unwrap();
                conteneur_lait.ajouter_lait(&mut boisson, 100.0);
            },
        }

        let monnaie = prochaine_requette.argent - prochaine_requette.commande.prix();
        let reponse = Reponse::Servie {
            client: prochaine_requette.client,
            boisson: boisson,
            monnaie,
        };

        reponse_send.send(reponse).unwrap()
    }
}
