#[derive(Debug)]
// Création de la structure "Livre" avec les champs nécessaires.
struct Livre {
    titre: String,
    auteur: String,
    annee: i16,
    disponible: bool,
}

// Implémentation des méthodes liées à Livre
impl Livre {
    //On crée simplement un nouveau Livre, en utilisant une fonction qui valide tous les inputs et permet de plus facilement créer des livres.
    fn new(titre: impl Into<String>, auteur: impl Into<String>, annee: i16) -> Livre {
        Livre {
            titre: titre.into(),
            auteur: auteur.into(),
            annee: annee,
            disponible: true,
        }
    }
}

fn main() {
    // On crée un vecteur de Livres
    let mut bibliotheque: Vec<Livre> = Vec::new();

    // Et on y push le résultat de la fonction "new", qui return directement un Livre.
    bibliotheque.push(Livre::new(
        "Le petit Prince",
        "Antoine de Saint-Exupéry",
        1943,
    ));

    // Débug, à retirer ensuite. Changer le nth(x) pour afficher un item différent du vecteur
    println!("{:?}", bibliotheque.into_iter().nth(0));
}
