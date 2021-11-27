// simple implementation of vec macro (vec!)
// for accessing elements inside the vector
struct Test {
    scores: i32,
}
fn main() {
    let my_scores = vec![
        Test { scores: 90 },
        Test { scores: 80 },
        Test { scores: 70 },
        Test { scores: 60 },
    ];

    for test in my_scores {
        println!("score= {:?}", test.scores);
    }
}
