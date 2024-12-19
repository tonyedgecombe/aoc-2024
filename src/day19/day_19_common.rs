pub fn parse_data(data: &[u8]) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let lines: Vec<&[u8]> = data
        .split(|&c| c ==b'\n')
        .filter(|line| !line.is_empty())
        .collect();

    let towels: Vec<&[u8]> = lines[0]
        .split(|&c| c == b',')
        .map(|towel| towel.trim_ascii())
        .collect();

    let designs = lines.into_iter().skip(1).collect();

    (towels, designs)
}

pub const EXAMPLE_DATA: &[u8] = b"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let (towels, designs) = parse_data(EXAMPLE_DATA);

        assert_eq!(towels.len(), 8);
        assert_eq!(towels[4], b"bwu");

        assert_eq!(designs.len(), 8);
        assert_eq!(designs[5], b"bwurrg")
    }

}