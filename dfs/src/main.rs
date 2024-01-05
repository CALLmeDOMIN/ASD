fn dfs(v: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    visited[v] = true;
    println!("{}", v + 1);
    for u in graph[v].iter() {
        if !visited[*u - 1] {
            dfs(*u - 1, visited, graph);
        }
    }
}

fn main() {
    let mut v: usize = 0;
    // let mut e: usize = 0;

    for (i, line) in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .enumerate()
    {
        if i == 0 {
            v = line.trim().parse::<usize>().unwrap();
        } else if i == 1 {
            // e = line.trim().parse::<usize>().unwrap();
        } else {
            break;
        }
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; v];

    for line in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .skip(2)
    {
        let mut iter = line.trim().split(", ");
        let u = iter.next().unwrap().parse::<usize>().unwrap();
        let v = iter.next().unwrap().parse::<usize>().unwrap();
        graph[u - 1].push(v);
    }

    println!("{:?}", graph);

    let mut visited: Vec<bool> = vec![false; v];

    dfs(0, &mut visited, &graph);

    println!("{:?}", visited);
}
