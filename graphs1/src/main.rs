fn weird_vertices(graph: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    for vertices in graph {
        for vs in vertices {
            if v.contains(vs) {
                continue;
            }
            v.push(*vs);
        }
    }

    v.sort();

    v
}

fn macierz_sasiadow(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let v: Vec<i32> = weird_vertices(graph);

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; v.len() + 1]; v.len() + 1];

    for i in 1..v.len() + 1 {
        matrix[0][i] = v[i - 1];
        matrix[i][0] = v[i - 1];
    }

    for vertices in graph.iter() {
        let vertex: i32 = vertices[0];
        for i in 1..vertices.len() {
            let edge = vertices[i];
            let index1: usize = matrix[0].iter().position(|&x| x == vertex).unwrap();
            let index2: usize = matrix[0].iter().position(|&x| x == edge).unwrap();
            matrix[index1][index2] = 1;
        }
    }

    matrix
}

fn macierz_incydencji(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut edges: i32 = 0;
    let vertices: Vec<i32> = weird_vertices(graph);

    for vertex in graph {
        for _ in vertex[1..].iter() {
            edges += 1;
        }
    }

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; edges as usize]; vertices.len()];

    let mut edge_count: usize = 0;
    for edges in graph {
        let vertex: usize = vertices.iter().position(|&x| x == edges[0]).unwrap();
        for v in edges[1..].iter() {
            let dest_vertex = vertices.iter().position(|&x| x == *v).unwrap();
            matrix[vertex][edge_count] = 1;
            matrix[dest_vertex][edge_count] = -1;
            edge_count += 1;
        }
    }

    matrix
}

fn main() {
    let mut data: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("file.txt")
        .expect("File not found!")
        .lines()
    {
        data.push(line.trim().to_string());
    }

    let mut graph: Vec<Vec<i32>> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }
        let values: Vec<i32> = line
            .split(", ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut pushed = false;
        for vertices in graph.iter_mut() {
            if vertices[0] == values[0] {
                vertices.push(values[1]);
                pushed = true;
                break;
            }
        }

        // if the graph wasnt directed
        // for vertices in graph.iter_mut() {
        //     if vertices[0] == values[1] {
        //         vertices.push(values[0]);
        //         pushed = true;
        //         break;
        //     }
        // }

        if !pushed {
            graph.push(vec![values[0], values[1]]);
        }
    }

    println!("Graph: {:?}", graph);

    println!("Matrix: ");
    let matrix = macierz_sasiadow(&graph);
    for line in matrix.iter() {
        for value in line.iter() {
            if value < &10 {
                print!(" {} ", value);
            } else {
                print!("{} ", value);
            }
        }
        println!("");
    }

    println!("Identity matrix: ");
    let matrix = macierz_incydencji(&graph);
    for line in matrix.iter() {
        for value in line.iter() {
            if value < &10 && value >= &0 {
                print!(" {} ", value);
            } else {
                print!("{} ", value);
            }
        }
        println!("");
    }
}
