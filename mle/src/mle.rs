pub fn map_items(prime: &i128, input_size: &i128) -> Vec<i128> {
    if prime < &(2 * *input_size) {
        panic!("Prime number {:?} must be greated than 2^input_size", prime)
    }
    let mut vec: Vec<i128> = Vec::new();
    let mut i: usize = 0;
    let mut start_val = 0;
    while i < (2 * *input_size) as usize {
        vec.push(start_val);
        start_val = start_val + 1;
        i = i + 1;
    }
    vec
}

pub fn decimal_to_binary_vec(decimal: &i128, input_size: usize) -> Vec<bool> {
    // Formats the binary number to a fixed length.
    let binary_string = format!("{:0>len$b}", decimal, len = input_size);
    binary_string
        .chars()
        .map(|characater: char| characater == '1')
        .collect()
}

pub fn chi_func(x: i128, w: bool) -> i128 {
    let w = w as i128;
    x * &w + (1 - &w) * (1 - x)
}

pub fn chi_func_product(field_vec: &Vec<i128>, binary_vec: &Vec<bool>) -> i128 {
    let chi_product = field_vec.iter().enumerate().fold(1, |product, (index, x)| {
        let w = binary_vec[index];
        product * chi_func(*x, w)
    });
    chi_product
}

pub fn stream_mle(elements: Vec<i128>, input_size: i128, f: Vec<i128>, p: i128) -> i128 {
    let mut mle_sum = 0;
    for w in 0..(2 * *&input_size as usize) {
        let f_w = f[w];
        let chi_func_product = chi_func_product(
            &elements,
            &decimal_to_binary_vec(&(w as i128), elements.len()),
        );
        mle_sum += f_w * chi_func_product;
    }
    mle_sum % p
}

pub fn lagrange_basis(x: &Vec<i128>, input_size: usize) -> Vec<i128> {
    if input_size == 1 {
        return vec![
            chi_func(x[input_size - 1], false),
            chi_func(x[input_size - 1], true),
        ];
    }

    return lagrange_basis(x, input_size - 1)
        .iter()
        .flat_map(|next_x| {
            vec![
                next_x * chi_func(x[input_size - 1], false),
                next_x * chi_func(x[input_size - 1], true),
            ]
        })
        .collect();
}

pub fn fast_mle(elements: Vec<i128>, input_size: i128, f: Vec<i128>, p: i128) -> i128 {
    let chi_vec = lagrange_basis(&elements, input_size as usize);

    assert_eq!(chi_vec.len(), f.len());

    let res = f
        .iter()
        .enumerate()
        .fold(0, |sum, (index, w)| sum + w * chi_vec[index]);
    res % p
}
