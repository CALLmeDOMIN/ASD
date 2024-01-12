fn dfs(vertex: i32, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, dfs_tree: &mut Vec<Vec<i32>>) {
    visited[vertex as usize] = true;

    let edges: &Vec<i32> = &graph[vertex as usize];

    for v in edges {
        if !visited[*v as usize] {
            dfs_tree[vertex as usize].push(*v);
            dfs(*v, graph, visited, dfs_tree);
        }
    }
}

fn bfs(
    vertex: i32,
    graph: &Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    bfs_tree: &mut Vec<Vec<i32>>,
    tree_visited: &mut Vec<i32>,
) {
    let mut queue: Vec<i32> = Vec::new();
    queue.push(vertex);
    tree_visited.push(vertex);

    while !queue.is_empty() {
        let u = queue.remove(0);

        if visited[u as usize] {
            continue;
        }

        visited[u as usize] = true;

        let edges: &Vec<i32> = &graph[u as usize];

        for v in edges {
            if !visited[*v as usize] {
                queue.push(*v);
            }
            if !tree_visited.contains(v) {
                bfs_tree[u as usize].push(*v);
                tree_visited.push(*v);
            }
        }
    }
}

fn main() {
    let file_content = std::fs::read_to_string("input.txt").expect("File not found!");
    let input: Vec<&str> = file_content.lines().collect::<Vec<_>>();

    let vertices: usize = input[0].parse::<usize>().unwrap();
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); vertices];

    for line in input[2..].iter() {
        let edge: Vec<&str> = line.split(" ").collect();
        let (a, b): (usize, usize) = (
            edge[0].parse::<usize>().unwrap() - 1,
            edge[1].parse::<usize>().unwrap() - 1,
        );

        graph[a].push(b as i32);
        graph[b].push(a as i32)
    }

    println!("{:?}", graph);

    let mut visited_dfs: Vec<bool> = vec![false; vertices];
    let mut dfs_tree: Vec<Vec<i32>> = vec![Vec::new(); vertices];

    dfs(0, &graph, &mut visited_dfs, &mut dfs_tree);

    println!("{:?}", dfs_tree);

    let mut visited_bfs: Vec<bool> = vec![false; vertices];
    let mut bfs_tree: Vec<Vec<i32>> = vec![Vec::new(); vertices];
    let mut bfs_tree_visited: Vec<i32> = Vec::new();

    bfs(
        0,
        &graph,
        &mut visited_bfs,
        &mut bfs_tree,
        &mut bfs_tree_visited,
    );

    println!("{:?}", bfs_tree);
}
