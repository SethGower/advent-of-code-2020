pub fn part1(input: String) -> Option<String> {
    let mut lines = input.lines();
    let timestamp: usize = lines.next()?.parse().ok()?;
    let earliest_departure: (usize, usize) = lines
        .next()?
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .map(|id| {
            let timestamp = ((timestamp / id) + 1) * id;
            (timestamp, id)
        })
        .min()?;
    println!("{:?}", earliest_departure);
    let resp = earliest_departure.1 * (earliest_departure.0 - timestamp);
    println!("{}", resp);
    Some(resp.to_string())
}
pub fn part2(input: String) -> Option<String> {
    Some(input)
}
