# Workshop Rust

Ce projet est une simulation d'un café.
Vous gérez un café dans la belle ville de Quebec. 

Lancez la simulation avec `cargo run`.

Il faut recevoir les commandes avec `requete_recv.recv()`, et envoyer les réponses avec 
`reponse_send.send()`.

Les commandes de boisson possibles:
- `CommandeBoisson::Expresso` : un expresso simple
- `CommandeBoisson::CafeAllonge` : un expresso avec de l'eau chaude
- `CommandeBoisson::CafeAuLait` : un expresso avec du lait

Il faut aussi rendre la monnaie au client. On peut avoir le prix d'une boisson avec la fonction 
`prix()`.
