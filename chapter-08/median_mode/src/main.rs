use median_mode::medmod;

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 3, 5, 5];
    let mode = medmod::mode(&vec);
    println!("{:?}", mode);

    let vec = vec![1, 1, 2, 2];
    let mode = medmod::mode(&vec);
    println!("{:?}", mode);

    let vec = vec![];
    let mode = medmod::mode(&vec);
    println!("{:?}", mode);

    let vec = vec![];
    let median = medmod::median(&vec);
    println!("{:?}", median);

    let vec = vec![9, 1, 3];
    let median = medmod::median(&vec);
    println!("{:?}", median);

    let vec = vec![9, 0, 8, 6, 7, 1, 2, 5, 3, 4];
    let median = medmod::median(&vec);
    println!("{:?}", median);
}
