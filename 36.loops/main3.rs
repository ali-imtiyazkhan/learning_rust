fn main(){
    'outer: for i in 1..=5 {
        'inner: for j in 1..=5 {
            if i == 3 && j == 3 {
                break 'outer;
            }
            println!("{} {}", i, j);
        }
    }
}

// expeced output 
// 1,1
// 1,2
// 1,3
// 1,4
// 1,5
// 2,1
// 2,2
// 2,3
// 2,4
// 2,5
// 3,1
// 3,2