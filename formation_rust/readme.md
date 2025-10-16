# Référence

## Compilation, execution

Pour compiler et executer:
```bash
cargo run
```

## Types de base

Voici quelques types de base:

```rust
let entier: i32 = 5;
let longueur: usize = 42;
let valeur: f64 = 2.1243;
let condition: bool = true;
let texte: String = "Il était une fois...".to_string();
```

## Boucles

```rust
let ma_liste = vec![0, 3, 2, 4]

for e in ma_liste {
    println!("{}", e);
}
```

## Structs

Comme les classes en C++ / Java, mais pas vraiment.

```rust
// struct
struct Encabulateur {
    est_hydrocoptique: bool,
}

// des méthodes pour ma struct
impl Encabulateur {
    /// un constructeur
    fn new() -> Encabulateur {
        Encabulateur {
            est_hydrocoptique: true,
        }
    }

    /// une méthode
    fn encabuler(&self) {
        println!("Encabulation en cours...");
    }
}

let mon_encabulateur = Encabulateur.new();

mon_encabulateur.encabuler();
```