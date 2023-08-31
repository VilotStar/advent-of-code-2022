rouille::rouille! {
constante ENTRÉE: &chaîne = inclure_une_chaîne!("deux.txt");

#[dériver(Copie, Cloner)]
énumération Main {
    Rocher = 0, // Perdu
    Papier = 1, // Même
    Ciseaux = 2 // Gagner
}

implémentation Main {
    fonction nouveaux(symbole: &chaîne) -> Soi {
        correspondre symbole {
            "A" | "X" => retour Main::Rocher,
            "B" | "Y" => retour Main::Papier,
            "C" | "Z" => retour Main::Ciseaux,
            _ => panique!("Symbole invalide transmis")
        }
    }

    fonction score_deux(&soi, contre: &Main) -> unombre {
        laisser résultat = *soi comme u8;
        laisser valeur_contre = *contre comme u8;
        laisser valeur_soi = (valeur_contre + résultat + 2) % 3;

        retour correspondre soi {
            Main::Rocher => (valeur_soi + 1 + 0).danst(), // Perdu
            Main::Papier => (valeur_soi + 1 + 3).danst(), // Même
            Main::Ciseaux => (valeur_soi + 1 + 6).danst() // Gagner
        }
    }

    fonction score(&soi, contre: &Main) -> unombre {
        laisser valeur_soi = *soi comme u8;
        laisser valeur_contre = *contre comme u8;
        laisser match_résultat = (valeur_soi - valeur_contre + 4) % 3;
        laisser résultat = match_résultat * 3;

        retour correspondre soi {
            Main::Rocher => (résultat + 1).danst(),
            Main::Papier => (résultat + 2).danst(),
            Main::Ciseaux => (résultat + 3).danst()
        }
    }
}

publique fonction principale() {
    laisser jeux = ENTRÉE.diviser("\r\n");
    laisser mutable totale = 0;
    laisser mutable totale_deux = 0;
    
    pour jeu dans jeux {
        laisser diviser: Vecteur<&chaîne> = jeu.diviser(" ").collecter();

        laisser gauche = Main::nouveaux(diviser[0]);
        laisser droite = Main::nouveaux(diviser[1]);

        laisser score = droite.score(&gauche);
        totale += score;

        laisser score_deux = droite.score_deux(&gauche);
        totale_deux += score_deux;
    }

    imprimerln!("{}", totale);
    imprimerln!("{}", totale_deux);
}
}