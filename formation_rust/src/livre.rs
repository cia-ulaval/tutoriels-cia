/// Une struct pour un livre
pub struct Livre {
    titre: String,
    auteur: String,
    pages: Vec<String>
}

impl Livre {
    /// Constructeur
    pub fn new(titre: &str, auteur: &str) -> Livre {
        Livre {
            titre: titre.to_string(),
            auteur: auteur.to_string(),
            pages: vec![],
        }
    }

    /// Ajoute une page au livre avec le contenu donné
    pub fn ajouter_page(&mut self, contenu: &str) {
        self.pages.push(contenu.to_string());
    }

    /// Retourne deux références immuables vers les champs internes
    pub fn get_titre(&self) -> &String {
        &self.titre
    }

    /// Retourne deux références immuables vers les champs internes
    pub fn get_auteur(&self) -> &String {
        &self.auteur
    }

    /// Retourne une référence à la liste de pages
    pub fn get_pages(&self) -> &Vec<String> {
        &self.pages
    }
}
