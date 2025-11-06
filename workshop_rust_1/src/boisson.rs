use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rand::Rng;

#[derive(Debug, Clone)]
pub enum CommandeBoisson {
    Espresso,
    CafeAllonge,
    CafeLatte,
}

impl CommandeBoisson {
    pub fn prix(&self) -> f32 {
        match self {
            CommandeBoisson::Espresso => 3.0,
            CommandeBoisson::CafeAllonge => 3.5,
            CommandeBoisson::CafeLatte => 5.0,
        }
    }
}

#[derive(Debug)]
pub struct Boisson {
    espresso_ml: f32,
    lait_ml: f32,
    eau_ml: f32,
    temp_c: f32,
}

const TEMP_AMBIANTE: f32 = 21.0;
const TEMP_BOISSON_CHAUDE: f32 = 67.0;
const ESPRESSO_ML: f32 = 27.0;
/// Temps d'extraction d'un espresso en secondes
const TEMPS_EXTRACTION_ESPRESSO: Duration = Duration::from_secs(1);

impl Boisson {
    /// Retourne un score entre -1.0 et 1.0, avec un commentaire
    pub fn evaluer_qualite(&self, commande: CommandeBoisson) -> (f32, String) {
        let mut commentaire = "*Boit sa boisson*".to_string();
        if self.est_vide() {
            return (-1.0, "Mais, c'est un verre vide???".to_string());
        }
        let ideal = match commande {
            CommandeBoisson::Espresso => Boisson {
                espresso_ml: ESPRESSO_ML,
                lait_ml: 0.0,
                eau_ml: 0.0,
                temp_c: TEMP_BOISSON_CHAUDE,
            },
            CommandeBoisson::CafeAllonge => Boisson {
                espresso_ml: ESPRESSO_ML,
                lait_ml: 0.0,
                eau_ml: 90.0, // ajout d'eau
                temp_c: TEMP_BOISSON_CHAUDE,
            },
            CommandeBoisson::CafeLatte => Boisson {
                espresso_ml: ESPRESSO_ML,
                lait_ml: 150.0, // ajout de lait
                eau_ml: 0.0,
                temp_c: 65.0,
            },
        };
        if self.temp_c > 70.0 {
            commentaire = "AIE! C'est brulant!".to_string();
        }
        let similarite = Self::evaluer_similarite(self, &ideal);
        if similarite > 0.9 {
            commentaire = "Mmh, c'est délicieux !".to_string();
        } else if similarite < 0.1 {
            commentaire = "Mais... c'est pas ce que j'ai commandé !".to_string();
        }
        return (similarite * 2.0 - 1.0, commentaire);
    }

    /// Calcule la similarité entre deux boissons (0.0 = très différent, 1.0 = identique)
    pub fn evaluer_similarite(b1: &Boisson, b2: &Boisson) -> f32 {
        const NUM_PARAMS: usize = 4;
        // Plages réalistes pour chaque ingrédient
        let plages = [
            (0.0, 60.0),  // espresso
            (0.0, 250.0), // lait
            (0.0, 400.0), // eau
            (10.0, 90.0), // température
        ];

        // Valeurs des deux boissons
        let v1: [f32; NUM_PARAMS] = [b1.espresso_ml, b1.lait_ml, b1.eau_ml, b1.temp_c];
        let v2: [f32; NUM_PARAMS] = [b2.espresso_ml, b2.lait_ml, b2.eau_ml, b2.temp_c];

        // Somme des différences normalisées
        let mut diff_totale = 0.0;
        for i in 0..NUM_PARAMS {
            let (min, max) = plages[i];
            let ecart = max - min;
            diff_totale += (v1[i] - v2[i]).abs() / ecart;
        }

        // Score final : 1.0 = identique, 0.0 = complètement différent
        let similarite = 1.0 - (diff_totale / NUM_PARAMS as f32);
        similarite.clamp(0.0, 1.0)
    }

    pub fn vide() -> Boisson {
        Boisson {
            espresso_ml: 0.0,
            lait_ml: 0.0,
            eau_ml: 0.0,
            temp_c: TEMP_AMBIANTE,
        }
    }

    fn est_vide(&self) -> bool {
        return self.espresso_ml == 0.0 && self.eau_ml == 0.0 && self.lait_ml == 0.0;
    }

    fn total_ml(&self) -> f32 {
        return self.espresso_ml + self.eau_ml + self.lait_ml;
    }
}

/// Une machine à espresso industrielle, capable de sortir les meilleurs cafés
/// Capable de faire N cafés en même temps
pub struct MachineEspresso<const N: usize> {
    positions: [Option<ExtractionEspresso>; N],
}

struct ExtractionEspresso {
    boisson: Boisson,
    instant_debut: Instant,
}

impl<const N: usize> MachineEspresso<N> {
    /// Crée une nouvelle machine à espresso
    /// - `places` : nombre de places dans la machine
    pub fn new() -> Self {
        Self {
            positions: [const { None }; N],
        }
    }

    /// Commence l'extration d'un espresso
    /// - `posiotion` : la position de la machine à utiliser. seulement une boisson peut ocupper une
    /// position à la fois
    /// - `boisson` : la boisson à mettre sous la machine
    ///
    /// Retourne un erreur si il y a déjà une boisson à cette position
    pub fn commencer_espresso(&mut self, posiotion: usize, boisson: Boisson) -> Result<(), ()> {
        if self.positions[posiotion].is_some() {
            return Err(());
        }
        self.positions[posiotion] = Some(ExtractionEspresso::new(boisson));
        Ok(())
    }

    /// Retire une boisson d'une position dans la machine
    /// - `posiotion` :
    ///
    /// Retourne une erreur si la position est encore en train d'extraire un espresso.
    /// Sinon, retourne la boisson si il y en a une à la position
    pub fn retirer_boisson(&mut self, pos: usize) -> Result<Boisson, ()> {
        match self.est_termine(pos) {
            Some(est_termine) => {
                if !est_termine {
                    return Err(());
                }
            }
            None => return Err(()),
        }
        // extraction est terminée
        let mut boisson = self.positions[pos].take().unwrap().boisson;
        Self::ajouter_espresso(&mut boisson);

        return Ok(boisson);
    }

    /// Si l'extraction d'espresso est terminé à cette position
    /// Retourne None si il n'y a pas d'espresso à cette position
    pub fn est_termine(&self, pos: usize) -> Option<bool> {
        if self.positions[pos].is_none() {
            return None;
        }
        let duration = Instant::now() - self.positions[pos].as_ref().unwrap().instant_debut;
        return Some(duration > TEMPS_EXTRACTION_ESPRESSO);
    }

    /// Ajoute de l'eau chaude à une boisson, avec un peut d'aléatoire.
    pub fn ajouter_eau_chaude(&self, boisson: &mut Boisson, ml: f32) {
        let mut rng = rand::rng();
        let eau_ml = ml + rng.random_range(-2.0..2.0);
        let ml_before = boisson.total_ml();
        boisson.eau_ml += eau_ml;
        boisson.temp_c =
            (boisson.temp_c * ml_before + TEMP_BOISSON_CHAUDE * eau_ml) / boisson.total_ml();
    }

    /// Ajoute de l'espresso à une boisson, avec un peut d'aléatoire.
    fn ajouter_espresso(boisson: &mut Boisson) {
        let mut rng = rand::rng();
        let espresso_ml = ESPRESSO_ML + rng.random_range(-2.0..2.0);
        let ml_before = boisson.total_ml();
        boisson.espresso_ml += espresso_ml;
        boisson.temp_c =
            (boisson.temp_c * ml_before + TEMP_BOISSON_CHAUDE * espresso_ml) / boisson.total_ml();
    }
}

impl ExtractionEspresso {
    fn new(boisson: Boisson) -> Self {
        Self {
            boisson,
            instant_debut: Instant::now(),
        }
    }
}

pub struct ConteneurLait {}

impl ConteneurLait {
    /// Ajoute du lait froid à la boisson
    pub fn ajouter_lait(&mut self, boisson: &mut Boisson, ml: f32) {
        // un petit delai de traitement...
        sleep(Duration::from_secs(1));
        let mut rng = rand::rng();
        let lait_ml = ml + rng.random_range(-2.0..2.0);
        let ml_before = boisson.total_ml();
        boisson.lait_ml += lait_ml;
        boisson.temp_c =
            (boisson.temp_c * ml_before + TEMP_AMBIANTE * lait_ml) / boisson.total_ml();
    }
}
