mod livre;
use livre::*;

fn main() {
    // TODO 1: Créer un livre qui s'appelle `mon_livre`
    //      (avec Livre::new(...))
    // avec nom "Crime et Châtiment" par "Fiodor Dostoïevski"

    let titre = mon_livre.get_titre();
    let auteur = mon_livre.get_auteur();
    let pages = mon_livre.get_pages();
    
    // TODO 2: Corrigez le problème ici...
    mon_livre.ajouter_page("Lorem ipsum dolor sit amet, consectetur adipiscing elit");


    let mut lettres = 0;
    // TODO 3: Comptez le nombre total de lettres dans les pages du livre
    //      (avec .len())

    println!("{} par {}", titre, auteur);
    println!("Nombre de lettres: {}", lettres);
}
