fn bfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, start: usize) {
    let mut queue: Vec<usize> = Vec::new();
    queue.push(start);

    while !queue.is_empty() {
        let u = queue.remove(0);
        visited[u] = true;
        println!("{}", u);

        for v in &graph[u] {
            if !visited[*v as usize] {
                queue.push(*v as usize);
            }
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
        graph[u].push(v);
    }

    println!("{:?}", graph);

    let mut visited: Vec<bool> = vec![false; v];

    bfs(&graph, &mut visited, 0);

    println!("{:?}", visited);
}
