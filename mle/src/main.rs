use mle::mle::{fast_mle, stream_mle};

fn main() {
    let prime = 5;

    let f: Vec<i128> = vec![1, 1, 2, 4];

    let elements: Vec<i128> = vec![2, 0];

    let streamed = stream_mle(elements, 2, f, prime);

    let f: Vec<i128> = vec![1, 1, 2, 4];

    let elements: Vec<i128> = vec![2, 0];

    let fast = fast_mle(elements, 2, f, prime);
    println!("{:#?}", streamed);
    println!("{:#?}", fast);
}
