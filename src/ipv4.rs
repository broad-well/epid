// We can represent an IPv4 address using one EPID3.
// The former has about 4 billion possible values, and the latter has more than 100 billion possible values.

// As (256^4)^(1/3) is between 1625 and 1626, we will select 1626 as the maximum word index possible.

use crate::wordlist::WORDS;

pub fn epid3_to_ipv4(epid: &str) -> Option<String> {
    parse_epid3(epid)
        .map(|comps| construct(&comps))
        .map(|ordinal| deconstruct(ordinal, 4))
        .map(|comps| format_ipv4(comps.as_slice()))
}

pub fn ipv4_to_epid3(ipv4: &str) -> Option<String> {
    parse_ipv4(ipv4)
        .map(|comps| construct(comps.iter()
            .map(|it| *it as u32)
            .collect::<Vec<u32>>().as_slice()))
        .map(|ordinal| deconstruct(ordinal, 3))
        .map(|comps| format_epid3(comps.as_slice()))
}

type OrdinalIPv4 = u32;
const IPV4_COMBS: usize = 4294967296; // 256^4

fn deconstruct(ordinal: OrdinalIPv4, len: usize) -> Vec<OrdinalIPv4> {
    let mut comp = vec![0; len];
    let mut rem = ordinal;
    let base = components_base(len);

    for place in 0..len {
        comp[len - place - 1] = rem % base;
        rem /= base;
        if rem == 0 {
            break;
        }
    }
    comp
}

fn construct(components: &[OrdinalIPv4]) -> OrdinalIPv4 {
    let base = components_base(components.len());

    components.iter().rev()
        .enumerate()
        .map(|(i, comp)| comp * base.pow(i as u32))
        .sum()
}

fn components_base(len: usize) -> u32 {
    (IPV4_COMBS as f32).powf(1f32 / (len as f32)).ceil() as u32
}

// FIXME need more specific errors?
fn parse_ipv4(ipv4: &str) -> Option<[u8; 4]> {
    let mut output: [u8; 4] = [0; 4];
    let comps: Vec<(usize, &str)> = ipv4.split(".").enumerate().collect();
    
    if comps.len() != 4 {
        return None;
    }

    for (i, comp) in comps {
        let result = comp.parse::<u8>();
        match result {
            Ok(component) => output[i] = component,
            Err(_) => return None
        }
    }

    Some(output)
}

fn parse_epid3(epid: &str) -> Option<[u32; 3]> {
    let mut out = [0u32; 3];
    let comps: Vec<(usize, &str)> = epid.split(".")
        .enumerate()
        .collect();

    if comps.len() != 3 {
        return None
    }

    for (i, comp) in comps {
        match WORDS.binary_search(&comp) {
            Ok(index) => out[i] = index as u32,
            Err(_) => return None
        }
    }
    
    Some(out)
}

fn format_ipv4(components: &[u32]) -> String {
    components.iter()
        .map(|comp| comp.to_string())
        .collect::<Vec<String>>()
        .join(".")
}

fn format_epid3(components: &[u32]) -> String {
    components.iter()
        .map(|i| WORDS[*i as usize])
        .collect::<Vec<&str>>()
        .join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deconstruct_low_number() {
        let components= deconstruct(40, 4);
        assert_eq!(components, &[0, 0, 0, 40]);
    }

    #[test]
    fn deconstruct_high_number() {
        let ordinal = 192 * 256u32.pow(3) + 168 * 256u32.pow(2) + 1;
        let components = deconstruct(ordinal, 4);
        assert_eq!(components, &[192, 168, 0, 1]);
    }

    #[test]
    fn deconstruct_three_len() {
        let ordinal = 525 * 1626u32.pow(2) + 231 * 1626 + 23;
        let components = deconstruct(ordinal, 3);
        assert_eq!(components, &[525, 231, 23]);
    }

    #[test]
    fn construct_four_len() {
        let comps = [192, 168, 0, 1];
        let ordinal = construct(&comps);
        assert_eq!(ordinal, 192 * 256u32.pow(3) + 168 * 256u32.pow(2) + 1);
    }

    #[test]
    fn construct_three_len() {
        let comps = [552, 131, 9];
        let ordinal = construct(&comps);
        assert_eq!(ordinal, 552 * 1626u32.pow(2) + 131 * 1626 + 9);
    }

    #[test]
    fn construct_deconstruct_inverse() {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        for i in 1..100 {
            let ordinal: u32 = rng.gen();
            let comps = deconstruct(ordinal, i % 10 + 3);
            let new_ord = construct(comps.as_slice());
            assert_eq!(ordinal, new_ord);
        }
    }

    #[test]
    fn parse_good_ipv4() {
        assert_eq!(parse_ipv4("192.168.1.1").unwrap(), [192, 168, 1, 1]);
    }

    #[test]
    fn parse_bad_ipv4s() {
        assert!(parse_ipv4("555.355.0.24").is_none());
        assert!(parse_ipv4("192.168.0").is_none());
        assert!(parse_ipv4("strong.curious.dolphin.man").is_none());
    }

    #[test]
    fn parse_good_epid3() {
        let vec: Vec<u32> = ["alerts", "baseline", "brazil"].iter()
            .map(|word| WORDS.binary_search(&word).unwrap() as u32)
            .collect();
        assert_eq!(&parse_epid3("alerts.baseline.brazil").unwrap(), vec.as_slice())
    }

    #[test]
    fn parse_bad_epid3s() {
        let bads = ["palabras.en.espanol", "alerts.baseline.baseline.alerts",
            "make.war", "ALERTS.BASELINE.BRAZIL", "other:white:divide", ""];

        for bad in bads.iter() {
            assert!(parse_epid3(bad).is_none());
        }
    }

    #[test]
    fn test_format_ipv4() {
        assert_eq!(format_ipv4(&[127, 0, 0, 1]), "127.0.0.1");
        assert_eq!(format_ipv4(&[192, 9, 24, 1]), "192.9.24.1");
    }

    #[test]
    fn test_format_epid3() {
        assert_eq!(format_epid3(&[252, 14, 98]), format!("{}.{}.{}", WORDS[252], WORDS[14], WORDS[98]));
        assert_eq!(format_epid3(&[90, 552, 1410]), format!("{}.{}.{}", WORDS[90], WORDS[552], WORDS[1410]));
    }

    #[test]
    fn test_epid3_ipv4_inverse() {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        for _ in 0..300 {
            let mut ipv4 = [0u8; 4];
            rng.fill(&mut ipv4);

            let ip = format_ipv4(ipv4.iter()
                .map(|i| *i as u32).collect::<Vec<u32>>().as_slice());

            let epid3 = ipv4_to_epid3(&ip).unwrap();
            let new_ip = epid3_to_ipv4(&epid3).unwrap();

            assert_eq!(ip, new_ip);
        }
    }
}