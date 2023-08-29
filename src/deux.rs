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
            selon contre {
                Main::Rocher => selon contre { // Perdu
                    Main::Rocher => renvoie 0 + 3,
                    Main::Papier => renvoie 0 + 1,
                    Main::Ciseaux => renvoie 0 + 2
                },
                Main::Papier => selon contre { // Même
                    Main::Rocher => renvoie 3 + 1,
                    Main::Papier => renvoie 3 + 2,
                    Main::Ciseaux => renvoie 3 + 3
                },
                Main::Ciseaux => selon contre { // Gagner
                    Main::Rocher => renvoie 6 + 2,
                    Main::Papier => renvoie 6 + 3,
                    Main::Ciseaux => renvoie 6 + 1
                }
            }
        }

        fonction score(&soi, contre: &Main) -> nsentier {
            soit valeur_soi = *soi as u8;
            soit valeur_contre = *contre as u8;
            soit résultat = (valeur_soi - valeur_contre + 4) % 3 * 3;

            renvoie selon soi {
                Main::Rocher => (résultat + 1).into(),
                Main::Papier => (résultat + 2).into(),
                Main::Ciseaux => (résultat + 3).into()
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