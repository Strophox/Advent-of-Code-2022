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

    let mut maximum = 1;
    for i in 0..l {
        for j in 0..l {
            let mut current = 1;
            let mut di = 1;
            while di < i && forest[i-di][j] < forest[i][j] {
                di += 1;
            }
            current *= di;

            let mut dip = 1;
            while i+dip < l-1 && forest[i+dip][j] < forest[i][j] {
                dip += 1;
            }
            current *= dip;

            let mut dj = 1;
            while dj < j && forest[i][j-dj] < forest[i][j] {
                dj += 1;
            }
            current *= dj;

            let mut djp = 1;
            while j+djp < l-1 && forest[i][j+djp] < forest[i][j] {
                djp += 1;
            }
            current *= djp;

            if maximum < current {
                maximum = current;
            }
        }
    }
    println!("Best scenic score is {maximum}");

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
