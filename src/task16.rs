#![allow(clippy::cast_lossless, clippy::cast_possible_truncation, clippy::too_many_lines)]

use crate::util::get_task;

// Every packet begins with a standard header: 
// the first three bits encode the packet version, 
// and the next three bits encode the packet type ID. 
// These two values are numbers; all numbers encoded in 
// any packet are represented as binary with the most 
// significant bit first. For example, a version encoded 
// as the binary sequence 100 represents the number 4.

#[derive(Debug, PartialEq, Eq, Clone)]
enum Payload {
    Literal(u64),
    Subpackets(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    version: u64,
    tag: u64,
    payload: Payload,
}

fn bslice_to_u64(bit_slice: &[u8]) -> u64 {
    let string = unsafe { String::from_utf8_unchecked(bit_slice.to_vec()) };
    u64::from_str_radix(&string, 2).unwrap_or(0)
}

fn read_packet(data: &[u8], mut offset: usize) -> (usize, Packet) {
    let version_bits = &data[offset..offset + 3];
    let version = bslice_to_u64(version_bits);

    offset += 3;

    let packet_type_bits = &data[offset..offset + 3];
    let tag = bslice_to_u64(packet_type_bits);

    offset += 3;

    if tag == 4 {
        let mut literal = Vec::new();
        loop {
            let block = &data[offset..offset + 5];
            // append all the bits of the block to the literal (except the flag bit)
            literal.extend_from_slice(&block[1..]);
            offset += 5;
            if block[0] == b'0' {
                break (offset, Packet {
                    version,
                    tag,
                    payload: Payload::Literal(bslice_to_u64(&literal)),
                });
            }
        }
    } else {
        let length_bit = data[offset] - b'0';
        offset += 1;

        let mut subpackets = Vec::new();

        if length_bit == 0 {
            let length_bits = &data[offset..offset + 15];
            let length = bslice_to_u64(length_bits);
            offset += 15;
            let subpackets_end = offset + length as usize;
            while offset < subpackets_end {
                let (read_end, subpacket) = read_packet(data, offset);
                offset = read_end;
                subpackets.push(subpacket);
            }
        } else {
            let packet_count_bits = &data[offset..offset + 11];
            let packet_count = bslice_to_u64(packet_count_bits);
            offset += 11;
            for _ in 0..packet_count {
                let (read_end, subpacket) = read_packet(data, offset);
                offset = read_end;
                subpackets.push(subpacket);
            }
        }

        (offset, Packet {
            version,
            tag,
            payload: Payload::Subpackets(subpackets),
        })
    }
}

fn sum_versions(p: &Packet) -> u64 {
    match &p.payload {
        Payload::Literal(_) => p.version,
        Payload::Subpackets(subpackets) => subpackets.iter().map(sum_versions).sum::<u64>() + p.version,
    }
}

fn eval_expr(packet: &Packet) -> u64 {
    match &packet.payload {
        Payload::Literal(literal) => *literal,
        Payload::Subpackets(subpackets) => match packet.tag {
            0 => subpackets.iter().map(eval_expr).sum(),
            1 => subpackets.iter().map(eval_expr).product(),
            2 => subpackets.iter().map(eval_expr).min().unwrap(),
            3 => subpackets.iter().map(eval_expr).max().unwrap(),
            5 => (eval_expr(&subpackets[0]) > eval_expr(&subpackets[1])) as u64,
            6 => (eval_expr(&subpackets[0]) < eval_expr(&subpackets[1])) as u64,
            7 => (eval_expr(&subpackets[0]) == eval_expr(&subpackets[1])) as u64,
            _ => panic!("unknown packet type"),
        }
    }
}

pub fn task16() {
    // io
    let input = get_task(16);
    let bits = input.chars().flat_map(|c| {
        let num = u8::from_str_radix(&c.to_string(), 16).unwrap();
        format!("{:04b}", num).chars().collect::<Vec<_>>()
    }).collect::<String>();
    let data = bits.as_bytes();

    let (_, root_packet) = read_packet(data, 0);

    // task 1

    println!("Task 1: {}", sum_versions(&root_packet));

    // task 2
    
    println!("Task 2: {}", eval_expr(&root_packet));
}
