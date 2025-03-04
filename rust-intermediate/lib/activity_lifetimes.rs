const mock_data: &'static str = include_str!("mockdata.csv");

struct Names<'a> {
    names: Vec<&'a str>,
}
struct Titles<'a> {
    titles: Vec<&'a str>,
}

pub fn activiti_lifetimes() {
    let full_data: Vec<_> = mock_data.split("\n").skip(1).collect();
    let _names: Vec<_> = full_data
        .iter()
        .filter_map(|id| id.split(",").nth(1))
        .collect();
    let names = Names { names: _names };
    let _titles: Vec<_> = full_data
        .iter()
        .filter_map(|id| id.split(",").nth(4))
        .collect();
    let titles = Titles { titles: _titles };
    let data = names.names.iter().zip(titles.titles.iter());
    for (names, titles) in data.take(10) {
        println!("Name = {} Title = {}", names, titles);
    }
}
