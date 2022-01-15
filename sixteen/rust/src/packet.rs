use crate::bits::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueOrPackets {
    Value(u64),
    Packets(Vec<Packet>),
}

#[derive(Debug, Clone, PartialEq)]
enum Length {
    Bits(usize),
    Packets(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    version: u8,
    r#type: u8,
    content: ValueOrPackets,
}

impl Packet {
    pub fn from_str(input: &str) -> Self {
        let bits = string_to_bits(input);
        Self::from_bits(&bits)
    }

    pub fn from_bits(bits: &[bool]) -> Self {
        let version = bits_to_num(&bits[0..3]) as u8;  
        let r#type = bits_to_num(&bits[3..6]) as u8;

        let content = match r#type {
            4 => ValueOrPackets::Value(parse_literal(&bits[6..]).0),
            typ => 
        }

        Self {
            version,
            r#type,
            content: ,
        }
    }

    fn parse_length_id(bits: &[bool]) -> Length {
        let is_length_in_bits = bits[0];

        if is_length_in_bits {
            Length::Bits(bits_to_num(&bits[1..16]) as usize)
        } else {
            Length::Packets(bits_to_num(&bits[1..11]) as usize)
        }
    }

    fn parse_packets(bits: &[bool], length: &Length) -> Vec<Packet> {
        
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        let input = "D2FE28";
        let packet = Packet::from_str(input);

        assert_eq!(packet.version, 6);
        assert_eq!(packet.r#type, 4);
        assert_eq!(packet.content, ValueOrPackets::Value(2021));
    }
}