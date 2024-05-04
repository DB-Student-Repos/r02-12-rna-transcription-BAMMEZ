#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

pub fn is_valid_strand(dna: &str, allowed: &str) -> Result<bool, usize> {
    for (n, c) in dna.chars().enumerate() {
        if !allowed.contains(c) {
            return Err(n);
        }
    }
    Ok(true)
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if is_valid_strand(dna, "CGAT")? {
            return Ok(Self {
                dna: dna.to_string(),
            });
        }
        Err(0)
    }

    pub fn into_rna(self) -> Rna {
        let rna = self
            .dna
            .clone()
            .chars()
            .map(|c| Dna::dna_to_rna(&c).unwrap())
            .collect::<String>();
        Rna::new(&rna).unwrap()
    }

    fn dna_to_rna(c: &char) -> Option<char> {
        match c {
            'G' => Some('C'),
            'C' => Some('G'),
            'T' => Some('A'),
            'A' => Some('U'),
            _ => None,
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if is_valid_strand(rna, "CGUA")? {
            return Ok(Self {
                rna: rna.to_string(),
            });
        }
        Err(0)
    }
}
