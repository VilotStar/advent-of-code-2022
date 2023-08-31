rdza::rdza! {
stały WEJŚCIE: &strunowy = zawierać_ciąg!("trzy.txt");

publiczny funkcjonować główny() {
    pozwalać plecaki = WEJŚCIE.podział("\r\n");
    pozwalać zmienny priorytet_suma: unumer = 0;

    pozwalać zmienny priorytet_suma_grupa: unumer = 0;
    pozwalać zmienny obecna_grupa: [Strunowy; 3] = Default::default();

    dle (indeks, plecak) w plecaki.wyliczać() {
        pozwalać (przedział_pierwszy, przedział_drugi) = plecak.podzielić_o_godz(plecak.długość() / 2);

        pozwalać znaki_wspólne: zwęglać = przedział_pierwszy.znaki()
            .filtr(|&znak| przedział_drugi.zawiera(znak))
            .nty(0).odwijaćsię();

        jeśli znaki_wspólne.jest_małe_litery() { // ord('a') == 97, 97 - 96 = 1
            priorytet_suma += znaki_wspólne jak unumer - 96;
        } inny {                         // ord('A') == 65, 65 - 64 = 1, ponieważ wielkie litery, 1 + 26 == 27
            priorytet_suma += znaki_wspólne jak unumer - 64 + 26;
        }

        obecna_grupa[indeks % 3] = plecak.do_sznurka();
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

        pozwalać znaki_wspólne_grupa: zwęglać = obecna_grupa[0].znaki()
            .filtr(|&znak| obecna_grupa[1].zawiera(znak) && obecna_grupa[2].zawiera(znak))
            .nty(0).odwijaćsię(); // szybciej niż w pętli

        jeśli znaki_wspólne.jest_małe_litery() { // ord('a') == 97, 97 - 96 = 1
            priorytet_suma_grupa += znaki_wspólne_grupa jak unumer - 96;
        } inny {                         // ord('A') == 65, 65 - 64 = 1, ponieważ wielkie litery, 1 + 26 == 27
            priorytet_suma_grupa += znaki_wspólne_grupa jak unumer - 64 + 26;
        }
    }

    wydrukowaćln!("{:?}", priorytet_suma);
    wydrukowaćln!("{:?}", priorytet_suma_grupa);
}
}