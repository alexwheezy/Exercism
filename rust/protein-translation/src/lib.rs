pub fn translate(rna: &str) -> Option<Vec<&str>> {
    if rna.is_empty() {
        return Some(vec![]);
    }

    let mut result = vec![];
    // split rna into chunks of 3
    for value in rna.as_bytes().chunks(3) {
        let protein = match value {
            b"AUG" => "Methionine",
            b"UUU" | b"UUC" => "Phenylalanine",
            b"UUA" | b"UUG" => "Leucine",
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => "Serine",
            b"UAU" | b"UAC" => "Tyrosine",
            b"UGU" | b"UGC" => "Cysteine",
            b"UGG" => "Tryptophan",
            b"UAA" | b"UAG" | b"UGA" => break,
            _ => return None,
        };
        result.push(protein);
    }
    Some(result)
}
