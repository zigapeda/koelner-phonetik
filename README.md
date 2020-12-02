# Kölner Phonetik

Kölner Phonetik or cologne phonetics is a phonetic algorithm like soundex, but specialized for german words.

Detailed description can be found on [Wikipedia](https://en.wikipedia.org/wiki/Cologne_phonetics) ([or in German](https://de.wikipedia.org/wiki/K%C3%B6lner_Phonetik))

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
koelner-phonetik = "0.1"
```

## Example
This example shows how to use Kölner Phonetik:
```rust
use koelner_phonetik;

let rust = String::from("Rust");
let phonetic = koelner_phonetik::calculate(&rust);

assert_eq!(phonetic, "782")
```

## License
`koelner-phonetik` is distributed under the terms of MIT License