rdza::rdza! {
stały WEJŚCIE: &ciąg = zawierać_ciąg!("trzy.txt");

publiczny funkcja główny() {
    niech plecaki = WEJŚCIE.podział("\r\n");
    niech zmienny priorytet_suma: ucałkowita = 0;

    niech zmienny priorytet_suma_grupa: ucałkowita = 0;
    niech zmienny obecna_grupa: [Ciąg; 3] = Default::default();

    dla (indeks, plecak) w plecaki.wyliczać() {
        niech (przedział_pierwszy, przedział_drugi) = plecak.podział_na(plecak.długość() / 2);

        niech znaki_wspólne: znakc = przedział_pierwszy.znakic()
            .filtr(|&znak| przedział_drugi.zawiera(znak))
            .nty(0).odpakuj();

        jeżeli znaki_wspólne.jest_niższy() { // ord('a') == 97, 97 - 96 = 1
            priorytet_suma += znaki_wspólne jako ucałkowita - 96;
        } inaczej {                         // ord('A') == 65, 65 - 64 = 1, ponieważ wielkie litery, 1 + 26 == 27
            priorytet_suma += znaki_wspólne jako ucałkowita - 64 + 26;
        }

        obecna_grupa[indeks % 3] = plecak.do_łańcucha();
        if indeks % 3 != 2 {
            kontynuować;
        }

        /* niech znaki_wspólne_grupa: znakc = obecna_grupa[0].znakic()
            .filtr(|&znak| {
                dla i w 1..3 {
                    if !obecna_grupa[i].zawiera(znak) {
                        zwróć fałsz;
                    }
                }
                prawda
            })
            .nty(0).odpakuj(); */

        niech znaki_wspólne_grupa: znakc = obecna_grupa[0].znakic()
            .filtr(|&znak| obecna_grupa[1].zawiera(znak) && obecna_grupa[2].zawiera(znak))
            .nty(0).odpakuj(); // szybciej niż w pętli

        jeżeli znaki_wspólne.jest_niższy() { // ord('a') == 97, 97 - 96 = 1
            priorytet_suma_grupa += znaki_wspólne_grupa jako ucałkowita - 96;
        } inaczej {                         // ord('A') == 65, 65 - 64 = 1, ponieważ wielkie litery, 1 + 26 == 27
            priorytet_suma_grupa += znaki_wspólne_grupa jako ucałkowita - 64 + 26;
        }
    }

    drukujln!("{:?}", priorytet_suma);
    drukujln!("{:?}", priorytet_suma_grupa);
}
}