use std::collections::HashMap;
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let is_validate = |c: char| !"ACGT".contains(c);
    if is_validate(nucleotide) {
        return Err(nucleotide);
    }

    let mut ncount: usize = 0;
    for value in dna.chars() {
        if is_validate(value) {
            return Err(value);
        }
        if value == nucleotide {
            ncount += 1;
        }
    }
    Ok(ncount)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for value in dna.chars() {
        *result.entry(value).or_insert(0) = count(value, dna)?;
    }
    Ok(result)
}
