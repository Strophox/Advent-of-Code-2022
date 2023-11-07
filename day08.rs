fn main() {
    println!("Opening door ...");
    let input = include_str!("08.txt");

    let forest: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let l = forest.len();
    let mut matrix: Vec<Vec<u32>> = input.lines().map(|line| line.bytes().map(|_| 0).collect()).collect();

    mark_visible(&forest, &mut matrix, |i:usize, j:usize| (i,j));
    mark_visible(&forest, &mut matrix, |i:usize, j:usize| (i,(l-1)-j));
    mark_visible(&forest, &mut matrix, |i:usize, j:usize| (j,i));
    mark_visible(&forest, &mut matrix, |i:usize, j:usize| ((l-1)-j,i));

    let total = matrix.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
    println!("The number of visible trees is {total}");

    println!("Part 2 ommitted in this program.");

    println!("Goodbye");
}

fn mark_visible(forest: &Vec<Vec<u8>>,
    matrix: &mut Vec<Vec<u32>>,
    transform: impl Fn(usize,usize)->(usize,usize))
{
    let l = matrix.len();
    for i in 0..l {
        let (a,b) = transform(i,0);
        let mut current = forest[a][b];
        matrix[a][b] = 1;
        for j in 1..l {
            let (a,b) = transform(i,j);
            if forest[a][b] > current {
                current = forest[a][b];
                matrix[a][b] = 1
            }
        }
    }
}
