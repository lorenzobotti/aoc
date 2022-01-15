pub fn parse_number_list(list: &str) -> Vec<usize> {
    list
        .trim()
        .split(",")
        .map(|num| num.trim().parse::<usize>())
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect()
}