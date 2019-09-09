#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    if num_discs == 1 {
        vec![(src, dst)]
    } else {
        let mut moves = hanoi(num_discs - 1, src, dst, aux);
        moves.push((src, dst));
        moves.append(&mut hanoi(num_discs - 1, aux, src, dst));
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // provided tests
    #[test]
    fn test_hanoi_1_disks() {
        let result = hanoi(1, Peg::A, Peg::B, Peg::C);
        assert_eq!(vec![(Peg::A, Peg::C)], result);
        assert_eq!(1, result.len());
    }

    // my own tests
    #[test]
    fn test_hanoi_4_disks() {
        let num_disk = 4;
        let result = hanoi(num_disk, Peg::A, Peg::B, Peg::C);
        println!("{:?}", result);
        assert_eq!(2_usize.pow(num_disk) - 1, result.len());
        assert_eq!(
            vec![
                (Peg::A, Peg::B),
                (Peg::A, Peg::C),
                (Peg::B, Peg::C),
                (Peg::A, Peg::B),
                (Peg::C, Peg::A),
                (Peg::C, Peg::B),
                (Peg::A, Peg::B),
                (Peg::A, Peg::C),
                (Peg::B, Peg::C),
                (Peg::B, Peg::A),
                (Peg::C, Peg::A),
                (Peg::B, Peg::C),
                (Peg::A, Peg::B),
                (Peg::A, Peg::C),
                (Peg::B, Peg::C),
            ],
            result
        );
    }
}