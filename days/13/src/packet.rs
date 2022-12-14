use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Packet {
    pub fn list(self) -> Option<Vec<Packet>> {
        match self {
            Packet::List(list) => Some(list),
            Packet::Num(_) => None,
        }
    }

    pub fn num(self) -> Option<u8> {
        match self {
            Packet::List(_) => None,
            Packet::Num(num) => Some(num),
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Packet::List(_))
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Packet::Num(_))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(list) => {
                let mut iter = list.iter();
                write!(f, "[",)?;
                if let Some(first_packet) = iter.next() {
                    write!(f, "{}", first_packet)?;
                    for packet in iter {
                        write!(f, ",{}", packet)?;
                    }
                }
                write!(f, "]",)?;
            }
            Packet::Num(num) => write!(f, "{}", num)?,
        }

        Ok(())
    }
}

impl FromStr for Packet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err(format!("The packet should have started with '[': {}", s));
        }
        if !s.ends_with(']') {
            return Err(format!("The packet should have started with '[': {}", s));
        }

        let s = &s[1..s.len() - 1];
        let mut stack = vec![];

        let mut many = vec![];
        let mut single = None;

        for ch in s.bytes() {
            match ch {
                b'0'..=b'9' => {
                    single = Some(match single.take() {
                        None => ch - b'0',
                        Some(val) => val * 10 + (ch - b'0'),
                    });
                }

                b',' => {
                    if let Some(value) = single.take() {
                        many.push(Packet::Num(value));
                    }
                }

                b'[' => {
                    stack.push((many, single));
                    many = vec![];
                    single = None;
                }

                b']' => {
                    if let Some(value) = single.take() {
                        many.push(Packet::Num(value));
                    }

                    let packet = Packet::List(many);
                    (many, single) = stack.pop().unwrap();
                    many.push(packet);
                }

                _ => return Err(format!("unexpected character: {}", ch as char)),
            }
        }

        assert!(
            stack.is_empty(),
            "the stack should have been empty: {:?}",
            stack
        );

        if let Some(value) = single.take() {
            many.push(Packet::Num(value));
        }

        Ok(Packet::List(many))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(l1), Packet::List(l2)) => {
                for (p1, p2) in l1.iter().zip(l2.iter()) {
                    let ordering = p1.cmp(p2);
                    if Ordering::Equal == ordering {
                        continue;
                    } else {
                        return ordering;
                    }
                }

                l1.len().cmp(&l2.len())
            }
            (Packet::List(_), Packet::Num(n2)) => self.cmp(&Packet::List(vec![Packet::Num(*n2)])),
            (Packet::Num(n1), Packet::List(_)) => Packet::List(vec![Packet::Num(*n1)]).cmp(other),
            (Packet::Num(n1), Packet::Num(n2)) => n1.cmp(n2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn from_str_empty_list() {
        let list = Packet::from_str("[]").unwrap();
        assert_eq!(Packet::List(vec![]), list,);
    }

    #[test]
    fn from_str_nested_empty_list() {
        let list = Packet::from_str("[[],[]]").unwrap();
        assert_eq!(
            Packet::List(vec![Packet::List(vec![]), Packet::List(vec![])]),
            list,
        );
    }

    #[test]
    fn from_str_flat_list() {
        let list = Packet::from_str("[1,2,3]").unwrap();
        assert_eq!(
            Packet::List(vec![Packet::Num(1), Packet::Num(2), Packet::Num(3)]),
            list,
        );
    }
}
