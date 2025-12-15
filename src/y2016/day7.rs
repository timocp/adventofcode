pub struct IPv7 {
    address: Vec<u8>,
    // boundary markers (start, end + 1) pairs of indexes into address
    supernets: Vec<(usize, usize)>,
    hypernets: Vec<(usize, usize)>,
}

impl From<&str> for IPv7 {
    fn from(s: &str) -> Self {
        let address = s.as_bytes().to_vec();

        let mut in_hypernet = false;
        let mut pos = 0;
        let mut supernets = vec![];
        let mut hypernets = vec![];

        // detect boundaries of address parts
        loop {
            if in_hypernet {
                let end_hypernet = address[pos..].iter().position(|&b| b == b']').unwrap();
                hypernets.push((pos, pos + end_hypernet));
                pos += end_hypernet + 1;
            } else if let Some(start_hypernet) = address[pos..].iter().position(|&b| b == b'[') {
                supernets.push((pos, pos + start_hypernet));
                pos += start_hypernet + 1;
            } else {
                supernets.push((pos, address.len()));
                break;
            }
            in_hypernet = !in_hypernet;
        }

        Self {
            address,
            supernets,
            hypernets,
        }
    }
}

impl IPv7 {
    // Transport-layer snooping
    // Any ABBA exists in supernet but none exist in hypernet
    fn supports_tls(&self) -> bool {
        self.supernets
            .iter()
            .any(|(start, end)| has_abba(&self.address[*start..*end]))
            && !self
                .hypernets
                .iter()
                .any(|(start, end)| has_abba(&self.address[*start..*end]))
    }

    // Super-secret listening
    // Any ABA exists in supernet and its inverse BAB exists in hypernet
    fn supports_ssl(&self) -> bool {
        for (start, end) in self.supernets.iter() {
            let abas = find_abas(&self.address[*start..*end]);
            for (a, b) in abas {
                for (start, end) in self.hypernets.iter() {
                    let part = &self.address[*start..*end];
                    if has_bab(part, a, b) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn has_abba(part: &[u8]) -> bool {
    (0..part.len() - 3)
        .map(|i| &part[i..i + 4])
        .any(|s| s[0] == s[3] && s[1] == s[2] && s[0] != s[1])
}

fn find_abas(part: &[u8]) -> Vec<(u8, u8)> {
    (0..part.len() - 2)
        .map(|i| &part[i..i + 3])
        .filter(|s| s[0] == s[2] && s[0] != s[1])
        .map(|s| (s[0], s[1]))
        .collect()
}

fn has_bab(part: &[u8], a: u8, b: u8) -> bool {
    (0..part.len() - 2).any(|i| part[i] == b && part[i + 1] == a && part[i + 2] == b)
}

pub fn parse_input(input: &str) -> Vec<IPv7> {
    input.lines().map(IPv7::from).collect()
}

pub fn part1(addresses: &[IPv7]) -> usize {
    addresses
        .iter()
        .filter(|address| address.supports_tls())
        .count()
}

pub fn part2(addresses: &[IPv7]) -> usize {
    addresses
        .iter()
        .filter(|address| address.supports_ssl())
        .count()
}

#[test]
fn test() {
    assert!(IPv7::from("abba[mnop]qrst").supports_tls());
    assert!(!IPv7::from("abcd[bddb]xyyx").supports_tls());
    assert!(!IPv7::from("aaaa[qwer]tyui").supports_tls());
    assert!(IPv7::from("ioxxoj[asdfgh]zxcvbn").supports_tls());

    assert!(IPv7::from("aba[bab]xyz").supports_ssl());
    assert!(!IPv7::from("xyx[xyx]xyx").supports_ssl());
    assert!(IPv7::from("aaa[kek]eke").supports_ssl());
    assert!(IPv7::from("zazbz[bzb]cdb").supports_ssl());
}
