rouille::rouille! {
constant ENTRÉE: &chaîne = inclure_chaîne!("deux.txt");

#[dériver(Copie, Cloner)]
énumération Main {
    Rocher = 0, // Perdu
    Papier = 1, // Même
    Ciseaux = 2 // Gagner
}

réalisation Main {
    fonction nouveaux(symbole: &chaîne) -> Soi {
        selon symbole {
            "A" | "X" => renvoie Main::Rocher,
            "B" | "Y" => renvoie Main::Papier,
            "C" | "Z" => renvoie Main::Ciseaux,
            _ => merde!("Symbole invalide transmis")
        }
    }

    fonction score_deux(&soi, contre: &Main) -> nsentier {
        soit résultat = *soi as u8;
        soit valeur_contre = *contre as u8;
        soit valeur_soi = (valeur_contre + résultat + 2) % 3;

        renvoie selon soi {
            Main::Rocher => (valeur_soi + 1 + 0).vers(), // Perdu
            Main::Papier => (valeur_soi + 1 + 3).vers(), // Même
            Main::Ciseaux => (valeur_soi + 1 + 6).vers() // Gagner
        }
    }

    fonction score(&soi, contre: &Main) -> nsentier {
        soit valeur_soi = *soi as u8;
        soit valeur_contre = *contre as u8;
        soit match_résultat = (valeur_soi - valeur_contre + 4) % 3;
        soit résultat = match_résultat * 3;

        renvoie selon soi {
            Main::Rocher => (résultat + 1).vers(),
            Main::Papier => (résultat + 2).vers(),
            Main::Ciseaux => (résultat + 3).vers()
        }
    } // Je pense s'il est possible d'utiliser un décalage de bits 
}

public fonction principale() {
    soit jeux = ENTRÉE.diviser("\r\n");
    soit mutable totale = 0;
    soit mutable totale_deux = 0;
    
    pour jeu de jeux {
        soit diviser: Tableau<&chaîne> = jeu.diviser(" ").collecter();

        soit gauche = Main::nouveaux(diviser[0]);
        soit droite = Main::nouveaux(diviser[1]);

        soit score = droite.score(&gauche);
        totale += score;

        soit score_deux = droite.score_deux(&gauche);
        totale_deux += score_deux;
    }

    affiche!("{}", totale);
    affiche!("{}", totale_deux);
}
}