use std::collections::HashMap;

pub fn parse_data(data: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data.lines().filter(|line| !line.is_empty()) {
        let mut tokens = line.split('-');
        let left = tokens.next().unwrap();
        let right = tokens.next().unwrap();

        map.entry(left).or_default().push(right);
        map.entry(right).or_default().push(left);
    }

    map
}


pub const EXAMPLE_DATA: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let graph = parse_data(EXAMPLE_DATA);
        assert_eq!(graph.len(), 16);

        let aq = graph.get("aq").unwrap();
        assert_eq!(aq.len(), 4);

        assert!(aq.contains(&"cg"));
        assert!(aq.contains(&"yn"));
        assert!(aq.contains(&"vc"));
        assert!(aq.contains(&"wq"));
    }
}