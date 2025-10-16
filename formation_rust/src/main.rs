mod livre;
use livre::*;

fn main() {
    // TODO 1: Créer un livre qui s'appelle `mon_livre`
    // votre code ici ...

    let titre_ref = mon_livre.get_titre();
    let auteur_ref = mon_livre.get_auteur();

    // TODO 2: Corriger l'erreur
    mon_livre.ajouter_page("Lorem ipsum dolor sit amet, consectetur adipiscing elit");

    let mots_total = 0;
    // TODO 3: Compter le nombre de mots.
    // votre code ici ...

    println!("{} par {}", titre_ref, auteur_ref);
    println!("Nombre de mots: {}", mots_total);
}
