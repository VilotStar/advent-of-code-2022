rouille::rouille! {
    constant ENTRÉE: &chaîne = inclure_chaîne!("deux.txt");

    énumération Main {
        Rocher,
        Papier,
        Ciseaux
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

        fonction score(&soi, contre: Main) -> nsentier {
            selon soi {
                Main::Rocher => selon contre {
                    Main::Rocher => renvoie 3 + 1,
                    Main::Papier => renvoie 0 + 1,
                    Main::Ciseaux => renvoie 6 + 1
                },
                Main::Papier => selon contre {
                    Main::Rocher => renvoie 6 + 2,
                    Main::Papier => renvoie 3 + 2,
                    Main::Ciseaux => renvoie 0 + 2
                },
                Main::Ciseaux => selon contre {
                    Main::Rocher => renvoie 0 + 3,
                    Main::Papier => renvoie 6 + 3,
                    Main::Ciseaux => renvoie 3 + 3
                }
            }
        } // Je pense s'il est possible d'utiliser un décalage de bits 
    }

    public fonction principale() {
        soit jeux = ENTRÉE.diviser("\r\n");
        soit mutable totale = 0;
        
        pour jeu de jeux {
            soit diviser: Tableau<&chaîne> = jeu.diviser(" ").collecter();

            soit gauche = Main::nouveaux(diviser[0]);
            soit droite = Main::nouveaux(diviser[1]);

            soit score = droite.score(gauche);
            totale += score;
        }

        affiche!("{}", totale);
    }
}