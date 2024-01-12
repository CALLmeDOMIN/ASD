struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.parent[x] = self.find(self.parent[x]);
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return false;
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
        } else {
            self.parent[y_root] = x_root;
            self.rank[x_root] += 1;
        }

        true
    }
}

fn union_find(t: &mut Vec<(usize, usize)>, l: &Vec<(usize, usize, i32)>, vertices: usize) {
    let mut uf: UnionFind = UnionFind::new(vertices);

    for &(a, b, _) in l {
        if uf.union(a, b) {
            t.push((a, b));
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("File not found!");
    let input: Vec<&str> = file.lines().collect::<Vec<_>>();

    let vertices: usize = input[0].parse::<usize>().unwrap();

    let mut t: Vec<(usize, usize)> = Vec::new();
    let mut l: Vec<(usize, usize, i32)> = Vec::new();

    for line in input[2..].iter() {
        let edge: Vec<&str> = line.split(" ").collect();

        let (a, b, c): (usize, usize, i32) = (
            edge[0].parse::<usize>().unwrap() - 1,
            edge[1].parse::<usize>().unwrap() - 1,
            edge[2].parse::<i32>().unwrap(),
        );

        l.push((a, b, c));
    }

    l.sort_by(|a, b| a.2.cmp(&b.2));

    union_find(&mut t, &l, vertices);

    let mut total_weight: i32 = 0;

    for &(a, b) in &t {
        total_weight += l
            .iter()
            .find(|&&(x, y, _)| (x == a && y == b) || (x == b && y == a))
            .unwrap()
            .2;
    }

    println!("{}", total_weight);
}
