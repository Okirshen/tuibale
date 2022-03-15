use std::{env, io::{BufReader, BufRead}, fs::File};
use prettytable::{Table, Row, Cell};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file given!");
    }
    let file = File::open(args[1].clone()).unwrap();
    let reader = BufReader::new(file);

    let mut table = Table::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let items = line.split(",");
        let list = items.map(|x| Cell::new(x.trim())).collect::<Vec<Cell>>();
        table.add_row(Row::new(list));
        // table.push(list);
    }

    table.printstd();
}
