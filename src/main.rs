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

    // Boucle principale du menu
    loop {
        println!("\n--- Bibliothèque ---");
        println!("1. Ajouter un livre");
        println!("2. Emprunter un livre");
        println!("3. Retourner un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres disponibles");
        println!("6. Quitter");

        let mut choix = String::new();
        println!("Votre choix:");
        std::io::stdin().read_line(&mut choix).unwrap();
        let choix: i32 = choix.trim().parse().unwrap_or(0);

        match choix {
            1 => {
                let titre = loop {
                    let mut input = String::new();
                    println!("Titre:");
                    std::io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim();
                    if !trimmed.is_empty() {
                        break trimmed.to_string();
                    }
                    println!("Le titre ne peut pas être vide.");
                };

                let auteur = loop {
                    let mut input = String::new();
                    println!("Auteur:");
                    std::io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim();
                    if !trimmed.is_empty() {
                        break trimmed.to_string();
                    }
                    println!("L'auteur ne peut pas être vide.");
                };

                let annee = loop {
                    let mut input = String::new();
                    println!("Année:");
                    std::io::stdin().read_line(&mut input).unwrap();

                    match input.trim().parse::<i16>() {
                        Ok(year) if (0..=2025).contains(&year) => break year,
                        Ok(_) => println!("Année invalide (0-2025)."),
                        Err(_) => println!("Veuillez entrer un nombre."),
                    }
                };

                bibliotheque.push(Livre::new(titre, auteur, annee));
                println!("Livre ajouté avec succès !");
            }
            2 => {
                // Emprunter un livre
                let mut titre = String::new();
                println!("Titre:");
                std::io::stdin().read_line(&mut titre).unwrap();
                emprunter_livre(&mut bibliotheque, titre.trim());
            }
            3 => {
                // Retourner un livre
                let mut titre = String::new();
                println!("Titre:");
                std::io::stdin().read_line(&mut titre).unwrap();
                retourner_livre(&mut bibliotheque, titre.trim());
            }
            4 => {
                // Afficher tous les livres
                afficher_livres(&mut bibliotheque);
            }
            5 => {
                // Afficher les livres disponibles
                afficher_livres_disponibles(&mut bibliotheque);
            }
            6 => {
                // Quitter
                println!("Au revoir !");
                break;
            }
            _ => {
                println!("Choix invalide, veuillez réessayer.");
            }
        }
    }
}
