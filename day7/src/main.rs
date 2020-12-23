use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//use petgraph::dot::Dot;
use petgraph::prelude::NodeIndex;
use petgraph::visit::EdgeRef; // source(), target()
use petgraph::Graph;

fn main() {
    let mut g = Graph::<String, usize>::new();
    let mut by_label: HashMap<String, NodeIndex> = HashMap::new();

    // Build the graph
    let f = File::open("input").unwrap();
    for (name, contents) in BufReader::new(f)
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse(&x))
    {
        let bag = *by_label
            .entry(name.clone())
            .or_insert_with(|| g.add_node(name.clone()));
        for (iname, icount) in contents {
            if icount == 0 {
                continue;
            }
            let inner = by_label
                .entry(iname.clone())
                .or_insert_with(|| g.add_node(iname.clone()));
            g.add_edge(bag, *inner, icount);
        }
    }
    //println!("{}", Dot::new(&g));

    let target = "shinygold";
    let gold = by_label.remove(target).unwrap();

    /* part 1
    let mut num_paths = 0;
    for (k, v) in by_label {
        //use petgraph::algo::has_path_connecting;
        if has_path_connecting(&g, v, gold, None) {
            num_paths += 1;
            println!("{} -> {}", k, target);
        }
    }
    println!("{}", num_paths);
    */

    // part 2
    println!("{}", num_bags(&g, gold));
}

fn num_bags(g: &Graph<String, usize>, origin: NodeIndex) -> usize {
    let mut subtotal = 0;
    for e in g.edges(origin) {
        subtotal += e.weight() * num_bags(g, e.target()) + e.weight();
    }
    //println!("{} {}", &g[origin], subtotal);
    subtotal
}

fn parse(line: &str) -> (String, Vec<(String, usize)>) {
    let mut parts = line.split("bags contain");
    let name = parts.next().unwrap().replace(" ", "");
    let mut contents = Vec::new();
    for inner in parts.next().unwrap().split(',') {
        let mut inner_parts = inner.trim().split(' ');
        // the unwrap_or(0) covers 'no'
        let count = inner_parts.next().unwrap().parse::<usize>().unwrap_or(0);
        let inner_name = String::from(inner_parts.next().unwrap()) + inner_parts.next().unwrap();
        contents.push((inner_name, count));
    }
    (name, contents)
}
