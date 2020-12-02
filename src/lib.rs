/// koelner_phonetik or cologne phonetics
/// is a phonetic algorithm like soundex,
/// but specialized for german words.
///
/// # Examples
///
/// ```
/// let rust = String::from("Rust");
/// let phonetic = koelner_phonetik::calculate(&rust);
/// 
/// assert_eq!(phonetic, "782")
/// ```
pub fn calculate(word: &str) -> String {
    let word = word.to_lowercase();
    let mut iter = word.chars().peekable();
    let mut retval = String::new();
    let mut last: Option<char> = None;
    let mut firsts = false;
    while let Some(c) = iter.next() {
        // quelle wikipedia: https://de.wikipedia.org/wiki/K%C3%B6lner_Phonetik
        let code = match c {
            // A, E, I, J, O, U, Y		0
            'a' | 'e' | 'i' | 'j' | 'o' | 'u' | 'y' | 'ä' | 'ö' | 'ü' => Some("0"),
            // H		–
            'h' => None,
            // B		1
            'b' => Some("1"),
            // P	nicht vor H		1
            'p' if iter.peek() != Some(&'h') => Some("1"),
            // D, T	nicht vor C, S, Z	2
            'd' | 't' if iter.peek() != Some(&'c') && iter.peek() != Some(&'s') && iter.peek() != Some(&'z') => Some("2"),
            // F, V, W		3
            'f' | 'v' | 'w' => Some("3"),
            // P	vor H	3
            'p' if iter.peek() == Some(&'h') => Some("3"),
            // G, K, Q		4
            'g' | 'k' | 'q' => Some("4"),
            // C	im Anlaut vor A, H, K, L, O, Q, R, U, X		4
            // assert_eq!(calculate("acl"), "045");
            'c' if (last == None || firsts == true) && (iter.peek() == Some(&'a') || iter.peek() == Some(&'h') || iter.peek() == Some(&'k')
                || iter.peek() == Some(&'l') || iter.peek() == Some(&'o') || iter.peek() == Some(&'q')
                || iter.peek() == Some(&'r') || iter.peek() == Some(&'u') || iter.peek() == Some(&'x')) => Some("4"),
            // C	vor A, H, K, O, Q, U, X außer nach S, Z		4
            'c' if last != Some('s') && last != Some('z') && (iter.peek() == Some(&'a') || iter.peek() == Some(&'h')
                || iter.peek() == Some(&'k') || iter.peek() == Some(&'o') || iter.peek() == Some(&'q')
                || iter.peek() == Some(&'u') || iter.peek() == Some(&'x')) => Some("4"),
            // X	nicht nach C, K, Q	48
            'x' if last != Some('c') && last != Some('k') && last != Some('q') => Some("48"),
            // L		5
            'l' => Some("5"),
            // M, N		6
            'm' | 'n' => Some("6"),
            // R		7
            'r' => Some("7"),
            // S, Z		8
            // C	nach S, Z	8
            // C	im Anlaut außer vor A, H, K, L, O, Q, R, U, X	8
            // C	nicht vor A, H, K, O, Q, U, X	8
            // D, T	vor C, S, Z	8
            // X	nach C, K, Q	8
            's' | 'z' | 'ß' | 'c' | 'd' | 't' | 'x' => Some("8"),
            ' ' => Some(" "),
            _ => None,
        };
        if c == ' ' {
            last = None;
        } else {
            if last == None && c == 's' {
                firsts = true;
            } else {
                firsts = false;
            }
            last = Some(c);
        }
        if let Some(code) = code {
            retval.push_str(code);
        }
    }
    let mut iter = retval.chars().peekable();
    let mut retval = String::new();
    while let Some(c) = iter.next() {
        if iter.peek() != Some(&c) {
            retval.push(c);
        }
    }
    let mut iter = retval.chars().peekable();
    let mut retval = String::new();
    while let Some(c) = iter.next() {
        if c == '0' && retval.len() == 0 {
            retval.push(c);
        } else if c != '0' {
            retval.push(c);
        }
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(calculate("Wikipedia"), "3412");
        assert_eq!(calculate("Breschnew"), "17863");
        assert_eq!(calculate("Erika"), "074");
        assert_eq!(calculate("Mustermann"), "682766");
        assert_eq!(calculate("Erika Mustermann"), "074 682766");
        assert_eq!(calculate("Müller-Lüdenscheidt"), "65752682");
        assert_eq!(calculate("Heinz Classen"), "068 4586");
        assert_eq!(calculate("Heinz-Classen"), "068586");
        assert_eq!(calculate("Acht"), "042");
        assert_eq!(calculate("Affe"), "03");
        assert_eq!(calculate("Ameise"), "068");
        assert_eq!(calculate("Ampel"), "0615");
        assert_eq!(calculate("Apfel"), "0135");
        assert_eq!(calculate("Apfelsine"), "013586");
        assert_eq!(calculate("Arzt"), "0782");
        assert_eq!(calculate("Ast"), "082");
        assert_eq!(calculate("Bagger"), "147");
        assert_eq!(calculate("Bahnhof"), "163");
        assert_eq!(calculate("Ball"), "15");
        assert_eq!(calculate("Banane"), "166");
        assert_eq!(calculate("Bank"), "164");
        assert_eq!(calculate("Baum"), "16");
        assert_eq!(calculate("Bär"), "17");
        assert_eq!(calculate("Biene"), "16");
        assert_eq!(calculate("Blume"), "156");
        assert_eq!(calculate("Buch"), "14");
        assert_eq!(calculate("Circus"), "8748");
        assert_eq!(calculate("Clown"), "4536");
        assert_eq!(calculate("Computer"), "46127");
        assert_eq!(calculate("Dach"), "24");
        assert_eq!(calculate("Dackel"), "245");
        assert_eq!(calculate("Delphin"), "2536");
        assert_eq!(calculate("Dino"), "26");
        assert_eq!(calculate("Domino"), "266");
        assert_eq!(calculate("Dose"), "28");
        assert_eq!(calculate("Drachen"), "2746");
        assert_eq!(calculate("Elefant"), "05362");
        assert_eq!(calculate("Ente"), "062");
        assert_eq!(calculate("Esel"), "085");
        assert_eq!(calculate("Fahrrad"), "372");
        assert_eq!(calculate("Fahne"), "36");
        assert_eq!(calculate("Fass"), "38");
        assert_eq!(calculate("Feder"), "327");
        assert_eq!(calculate("Fenster"), "36827");
        assert_eq!(calculate("Ferkel"), "3745");
        assert_eq!(calculate("Fisch"), "38");
        assert_eq!(calculate("Fuchs"), "348");
        assert_eq!(calculate("Gabel"), "415");
        assert_eq!(calculate("Geige"), "44");
        assert_eq!(calculate("Giraffe"), "473");
        assert_eq!(calculate("Gitarre"), "427");
        assert_eq!(calculate("Glas"), "458");
        assert_eq!(calculate("Gras"), "478");
        assert_eq!(calculate("Hahn"), "06");
        assert_eq!(calculate("Hand"), "062");
        assert_eq!(calculate("Hase"), "08");
        assert_eq!(calculate("Haus"), "08");
        assert_eq!(calculate("Hemd"), "062");
        assert_eq!(calculate("Hexe"), "048");
        assert_eq!(calculate("Honig"), "064");
        assert_eq!(calculate("Huhn"), "06");
        assert_eq!(calculate("Hund"), "062");
        assert_eq!(calculate("Hut"), "02");
        assert_eq!(calculate("Igel"), "045");
        assert_eq!(calculate("Indianer"), "06267");
        assert_eq!(calculate("Insel"), "0685");
        assert_eq!(calculate("Jacke"), "04");
        assert_eq!(calculate("Jäger"), "047");
        assert_eq!(calculate("Jaguar"), "047");
        assert_eq!(calculate("Jo-Jo"), "0");
        assert_eq!(calculate("Junge"), "064");
        assert_eq!(calculate("Käfer"), "437");
        assert_eq!(calculate("Kamel"), "465");
        assert_eq!(calculate("Känguru"), "4647");
        assert_eq!(calculate("Katze"), "48");
        assert_eq!(calculate("Kerze"), "478");
        assert_eq!(calculate("Kirsche"), "478");
        assert_eq!(calculate("Knopf"), "4613");
        assert_eq!(calculate("Koch"), "44");
        assert_eq!(calculate("König"), "464");
        assert_eq!(calculate("Kuchen"), "446");
        assert_eq!(calculate("Kuh"), "4");
        assert_eq!(calculate("Krankenhaus"), "476468");
        assert_eq!(calculate("Krokodil"), "47425");
        assert_eq!(calculate(""), "");
        assert_eq!(calculate("Lama"), "56");
        assert_eq!(calculate("Lampe"), "561");
        assert_eq!(calculate("Lehrer"), "577");
        assert_eq!(calculate("Leiter"), "527");
        assert_eq!(calculate("Libelle"), "515");
        assert_eq!(calculate("Licht"), "542");
        assert_eq!(calculate("Lineal"), "565");
        assert_eq!(calculate("Löffel"), "535");
        assert_eq!(calculate("Löwe"), "53");
        assert_eq!(calculate("Lupe"), "51");
        assert_eq!(calculate("Maus"), "68");
        assert_eq!(calculate("Messer"), "687");
        assert_eq!(calculate("Micky-Maus"), "6468");
        assert_eq!(calculate("Mond"), "662");
        assert_eq!(calculate("Muschel"), "685");
        assert_eq!(calculate("Mütze"), "68");
        assert_eq!(calculate("Nadel"), "625");
        assert_eq!(calculate("Nase"), "68");
        assert_eq!(calculate("Nashorn"), "6876");
        assert_eq!(calculate("Nest"), "682");
        assert_eq!(calculate("Netz"), "68");
        assert_eq!(calculate("Nuss"), "68");
        assert_eq!(calculate("Nutella"), "625");
        assert_eq!(calculate("Obst"), "0182");
        assert_eq!(calculate("Ofen"), "036");
        assert_eq!(calculate("Oma"), "06");
        assert_eq!(calculate("Omnibus"), "0618");
        assert_eq!(calculate("Opa"), "01");
        assert_eq!(calculate("Ordner"), "07267");
        assert_eq!(calculate("Osterhase"), "08278");
        assert_eq!(calculate("Paket"), "142");
        assert_eq!(calculate("Palme"), "156");
        assert_eq!(calculate("Papagei"), "114");
        assert_eq!(calculate("Pilz"), "158");
        assert_eq!(calculate("Pinguin"), "1646");
        assert_eq!(calculate("Pinsel"), "1685");
        assert_eq!(calculate("Post"), "182");
        assert_eq!(calculate("Pulli"), "15");
        assert_eq!(calculate("Qualle"), "45");
        assert_eq!(calculate("Quelle"), "45");
        assert_eq!(calculate("Rad"), "72");
        assert_eq!(calculate("Radio"), "72");
        assert_eq!(calculate("Rakete"), "742");
        assert_eq!(calculate("Regenschirm"), "746876");
        assert_eq!(calculate("Reh"), "7");
        assert_eq!(calculate("Ritter"), "727");
        assert_eq!(calculate("Roller"), "757");
        assert_eq!(calculate("Rose"), "78");
        assert_eq!(calculate("Säge"), "84");
        assert_eq!(calculate("See"), "8");
        assert_eq!(calculate("Sofa"), "83");
        assert_eq!(calculate("Sonne"), "86");
        assert_eq!(calculate("Tasse"), "28");
        assert_eq!(calculate("Telefon"), "2536");
        assert_eq!(calculate("Tiger"), "247");
        assert_eq!(calculate("Tisch"), "28");
        assert_eq!(calculate("Topf"), "213");
        assert_eq!(calculate("Traktor"), "27427");
        assert_eq!(calculate("Turm"), "276");
        assert_eq!(calculate("Ufo"), "03");
        assert_eq!(calculate("Uhr"), "07");
        assert_eq!(calculate("Uhu"), "0");
        assert_eq!(calculate("Unfall"), "0635");
        assert_eq!(calculate("Unterhemd"), "062762");
        assert_eq!(calculate("Unterhose"), "06278");
        assert_eq!(calculate("Vase"), "38");
        assert_eq!(calculate("Verband"), "37162");
        assert_eq!(calculate("Vogel"), "345");
        assert_eq!(calculate("Wal"), "35");
        assert_eq!(calculate("Wald"), "352");
        assert_eq!(calculate("Wasser"), "387");
        assert_eq!(calculate("Wolke"), "354");
        assert_eq!(calculate("Werkstatt"), "374822");
        assert_eq!(calculate("Würfel"), "3735");
        assert_eq!(calculate("Wurm"), "376");
        assert_eq!(calculate("Xaver"), "4837");
        assert_eq!(calculate("Xylophon"), "48536");
        assert_eq!(calculate("Yacht"), "042");
        assert_eq!(calculate("Yak"), "04");
        assert_eq!(calculate("Yeti"), "02");
        assert_eq!(calculate("Yo-Yo"), "0");
        assert_eq!(calculate("Ypsilon"), "01856");
        assert_eq!(calculate("Zange"), "864");
        assert_eq!(calculate("Zaun"), "86");
        assert_eq!(calculate("Zebra"), "817");
        assert_eq!(calculate("Zelt"), "852");
        assert_eq!(calculate("Ziege"), "84");
        assert_eq!(calculate("Zitrone"), "8276");
        assert_eq!(calculate("Zoo"), "8");
        assert_eq!(calculate("Zug"), "84");
        assert_eq!(calculate("Ähre"), "07");
        assert_eq!(calculate("Äpfel"), "0135");
        assert_eq!(calculate("Ärmel"), "0765");
        assert_eq!(calculate("Öfen"), "036");
        assert_eq!(calculate("Öl"), "05");
        assert_eq!(calculate("Ölsardine"), "058726");
        assert_eq!(calculate("Überholverbot"), "01753712");
        assert_eq!(calculate("Überraschungsei"), "0178648");
        assert_eq!(calculate("Auge"), "04");
        assert_eq!(calculate("Auto"), "02");
        assert_eq!(calculate("Chinese"), "468");
        assert_eq!(calculate("Chinesisch"), "4688");
        assert_eq!(calculate("Christbaum"), "478216");
        assert_eq!(calculate("Ei"), "0");
        assert_eq!(calculate("Eichhörnchen"), "047646");
        assert_eq!(calculate("Eidechse"), "0248");
        assert_eq!(calculate("Eis"), "08");
        assert_eq!(calculate("Eisberg"), "08174");
        assert_eq!(calculate("Eule"), "05");
        assert_eq!(calculate("Euro"), "07");
        assert_eq!(calculate("Pfanne"), "136");
        assert_eq!(calculate("Pfau"), "13");
        assert_eq!(calculate("Pferd"), "1372");
        assert_eq!(calculate("Pfeil"), "135");
        assert_eq!(calculate("Qualle"), "45");
        assert_eq!(calculate("Quelle"), "45");
        assert_eq!(calculate("Qualm"), "456");
        assert_eq!(calculate("Schaf"), "843");
        assert_eq!(calculate("Schere"), "847");
        assert_eq!(calculate("Schiff"), "843");
        assert_eq!(calculate("Schuh"), "84");
        assert_eq!(calculate("Schach"), "844");
        assert_eq!(calculate("Schaschlik"), "84854");
        assert_eq!(calculate("Sache"), "84");
        assert_eq!(calculate("Sparschwein"), "817836");
        assert_eq!(calculate("Speer"), "817");
        assert_eq!(calculate("Spiegel"), "8145");
        assert_eq!(calculate("Spinne"), "816");
        assert_eq!(calculate("Star"), "827");
        assert_eq!(calculate("Stein"), "826");
        assert_eq!(calculate("Stern"), "8276");
    }
}
