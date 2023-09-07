fn main() {
    let input = include_str!("08.txt");

    let trees: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let l = trees.len();

    let mut total_visible = 0;
    let mut best_score = 1;
    'T: for i in 0..l {
        for j in 0..l {
            let mut visible = false;
            let mut score = 1;

            let mut di = 1;
            score *= loop {
                if l-1 < i+di {
                    visible = true;
                    break di-1;
                } else if trees[i+di][j] <= trees[i][j] {
                    break di;
                } else { di += 1; }
            };
let tempi = di;
            di = 1;
            score *= loop {
                if i < di { // i-di < 0
                    visible = true;
                    break di-1;
                } else if trees[i-di][j] <= trees[i][j] {
                    break di;
                } else { di += 1; }
            };

            let mut dj = 1;
            score *= loop {
                if l-1 < j+dj {
                    visible = true;
                    break dj-1;
                } else if trees[i][j+dj] <= trees[i][j] {
                    break dj;
                } else { dj += 1; }
            };
let tempj = dj;
            dj = 1;
            score *= loop {
                if j < dj { // j-dj < 0
                    visible = true;
                    break dj-1;
                } else if trees[i][j-dj] <= trees[i][j] {
                    break dj;
                } else { dj += 1; }
            };

// print!("{:?} visibility: {visible} ", (i,j));
// if i > 0 { println!("{:?} -> {score}", (tempi,di,tempj,dj)); }
            if visible {
                total_visible += 1;
            }
            if best_score < score {
                best_score = score;
            }
            // break 'T;
        }
    }
    println!("UNFINISHED");
    println!("Number of visible trees is {total_visible}");
    println!("Best scenic score is {best_score}");
}
