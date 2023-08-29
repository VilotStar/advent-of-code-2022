rdza::rdza! {
stały WEJŚCIE: &ciąg = zawierać_ciąg!("trzy.txt");

publiczny funkcja główny() {
    niech plecaki = WEJŚCIE.podział("\r\n");
    niech zmienny priorytet_suma: ucałkowita = 0;

    dla plecak w plecaki {
        niech (przedział_pierwszy, przedział_drugi) = plecak.podział_na(plecak.długość() / 2);

        niech znaki_wspólne: znakc = przedział_pierwszy.znakic()
            .filtr(|&znak| przedział_drugi.zawiera(znak))
            .nty(0).odpakuj();

        if znaki_wspólne.jest_niższy() { // ord('a') == 97, 97 - 96 = 1
            priorytet_suma += znaki_wspólne as ucałkowita - 96;
        } else {                                // ord('A') == 65, 65 - 64 = 1, ponieważ wielkie litery, 1 + 26 == 27
            priorytet_suma += znaki_wspólne as ucałkowita - 64 + 26;
        }
    }

    drukujln!("{:?}", priorytet_suma);
}
}