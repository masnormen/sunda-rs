use fancy_regex::{Regex, Captures};

pub fn capture_latin(text: &String) -> Vec<Captures> {
    let regex = Regex
        ::new(r"(?<digits>[\d]+)|(?<punctuations>[^a-zA-Z\d\x{00C0}-\x{00FF}]+)|(?<consonant_main>ng|ny|kh|sy|[kgcjtdnpbmyrlwshfqvxz])?(?<consonant_sonorant>[ylr])?(?<vowel>e`|eu|[aiueoéè])(?<consonant_rarangken>(?:ng|[rh])(?!e`|eu|[aiueoéè]))?(?<consonant_final>(?:n(?![gy])|k(?!h)|s(?!y)|ny|kh|sy|[gcjtdpbmylwfvqxz])(?!e`|eu|[aiueoéè]))?|(?<consonant_standalone>ng|ny|kh|sy|[kgcjtdnpbmyrlwshfqvxz])")
        .unwrap();

    regex
        .captures_iter(text)
        .map(|capture| capture.unwrap())
        .collect()
}

pub fn capture_sunda(text: &String) -> Vec<Captures> {
    let regex = Regex
        ::new(r"(?:[|])?(?<angka>[\x{1BB0}])(?:[|])?|(?<not_sunda>[^\x{1B80}-\x{1BA8}\x{1BAE}-\x{1BB9}]+)|(?:(?<ngalagena>[\x{1B8A}-\x{1BA0}\x{1BAE}\x{1BAF}])(?<rarangken_sonorant>[\x{1B1}-\x{1BA3}])?(?<rarangken_vowel>[\x{1BA4}-\x{1BAA}])?|(?<swara>[\x{1B83}-\x{1B89}]))(?<rarangken_final>[\x{1B80}-\x{1B82}])?")
        .unwrap();

    regex
        .captures_iter(text)
        .map(|capture| capture.unwrap())
        .collect()
}
