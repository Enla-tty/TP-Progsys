#[derive(Debug, Clone)]
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

// Emprunter un livre. N'a besoin que du titre du livre et de la bibliothèque où chercher.
fn emprunter_livre(bibliotheque: &mut Vec<Livre>, titre_recherche: &str) {
    for livre in bibliotheque.iter_mut() {
        if livre.titre == titre_recherche {
            if livre.disponible {
                livre.disponible = false;
                println!("{} emprunté !", titre_recherche);
            } else {
                println!("{} est déjà emprunté !", titre_recherche);
            }
            break;
        }
    }
}

// Retourner un livre. N'a besoin que du titre du livre et de la bibliothèque où chercher.
fn retourner_livre(bibliotheque: &mut Vec<Livre>, titre_recherche: &str) {
    for livre in bibliotheque.iter_mut() {
        if livre.titre == titre_recherche {
            if livre.disponible == false {
                livre.disponible = true;
                println!("{} retourné !", titre_recherche);
            } else {
                println!("{} est déjà disponible !", titre_recherche);
            }
            break;
        }
        println!("{} non trouvé !", titre_recherche);
    }
}

fn afficher_livres(bibliotheque: &mut Vec<Livre>) {
    println!("\n#######################\n#       Livres:       #\n#######################");
    for livre in bibliotheque.iter_mut() {
        println!(
            " Titre: {} \n Auteur: {}\n Année: {}\n Disponible: {}\n",
            livre.titre, livre.auteur, livre.annee, livre.disponible
        )
    }
}

fn afficher_livres_disponibles(bibliotheque: &mut Vec<Livre>) {
    println!("\n#######################\n# Livres Disponibles: #\n#######################");
    for livre in bibliotheque.iter_mut() {
        if livre.disponible == true {
            println!(
                " Titre: {} \n Auteur: {}\n Année: {}\n ####### \n",
                livre.titre, livre.auteur, livre.annee
            )
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
    println!("{:?}", bibliotheque.clone().into_iter().nth(0));

    emprunter_livre(&mut bibliotheque, "Le petit Prince");
    retourner_livre(&mut bibliotheque, "Le petit Prince");
    retourner_livre(&mut bibliotheque, "Le petit Prince");
    retourner_livre(&mut bibliotheque, "Robinson Crusoé");
    emprunter_livre(&mut bibliotheque, "Robinson Crusoé");
    afficher_livres(&mut bibliotheque);
    emprunter_livre(&mut bibliotheque, "Le petit Prince");
    afficher_livres_disponibles(&mut bibliotheque);
}
