use std::io;

fn main() {
    let mut input = String::new();
    println!("Separe a expressão com espaços.\n Exemplo: 10 + 2 * 3\n");
    loop {
        io::stdin().read_line(&mut input).expect("deu merda");
        if input.contains("exit") {
            break;
        }
        print!("\n");

        println!(
            "Result: {}\n",
            calculate_basic(get_numbers(&input), get_operators(&input))
        );
        input = String::from("");
    }
}

#[allow(unused_variables, unused_mut)]
fn calculate_basic(nums: Vec<f64>, op: String) -> f64 {
    let mut result = 0.0;
    let op_len = op.len();
    let n_len = nums.len();

    if !op.contains('*') && !op.contains('/') {
        for i in 0..op_len {
            if op.chars().nth(i).unwrap() == '+' {
                result += nums.get(i).unwrap();
                if i == op_len - 1 {
                    result += nums.get(i + 1).unwrap();
                }
            } else if op.chars().nth(i).unwrap() == '-' {
                if i == op_len - 1 {
                    result -= nums.get(i + 1).unwrap();
                }
                if i == 0 {
                    result += nums.get(i).unwrap();
                } else {
                    result -= nums.get(i).unwrap();
                }
            }
        }
    } else {
        for i in 0..op_len {
            if op.chars().nth(i).unwrap() == '*' {
                result += nums.get(i).unwrap() * nums.get(i + 1).unwrap();
            } else if op.chars().nth(i).unwrap() != '*' {
                if op.chars().nth(i).unwrap() == '+' && i == op_len - 1 {
                    result += nums.get(i + 1).unwrap();
                } else if op.chars().nth(i).unwrap() == '+' {
                    result += nums.get(i).unwrap();
                }

                if op.chars().nth(i).unwrap() == '-' && i == op_len - 1 {
                    result -= nums.get(i + 1).unwrap();
                } else if op.chars().nth(i).unwrap() == '-' {
                    result -= nums.get(i).unwrap();
                }

                if op.chars().nth(i).unwrap() == '/' && i == op_len - 1 {
                    result /= nums.get(i + 1).unwrap();
                } else if op.chars().nth(i).unwrap() == '/' {
                    result /= nums.get(i).unwrap();
                }
            }
        }
    }

    /* let (mut a, mut b, mut c, mut d) = porra(op); */
    // println!("{}{}{}{}  {}", a, b, c, d, op_len);

    // for n in 2..n_len {
    //     // if c != 0 {
    //     //     result /= nums.get(n).unwrap();
    //     //     c -= 1;
    //     // } else if d != 0 {
    //     //     result *= nums.get(n).unwrap();
    //     //     d -= 1;
    //     if a != 0 {
    //         result += nums.get(n).unwrap();
    //         a -= 1;
    //     } else if b != 0 {
    //         result -= nums.get(n).unwrap();
    //         b -= 1;
    //     }
    // }

    result
}

fn get_numbers(input: &String) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();

    for ch in input.split_whitespace() {
        let p = ch.parse::<f64>();
        match p {
            Ok(p) => numbers.push(p),
            _ => (),
        }
    }

    numbers
}

fn get_operators(input: &String) -> String {
    let mut op_result = String::new();

    for ch in input.chars() {
        match ch {
            '+' => op_result.push('+'),
            '-' => op_result.push('-'),
            '/' => op_result.push('/'),
            '*' => op_result.push('*'),
            _ => (),
        }
    }

    op_result
}

// fn porra(mut op: String) -> (usize, usize, usize, usize) {
//     let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
//     op.remove(0);

//     for ch in op.chars() {
//         match ch {
//             '+' => a += 1,
//             '-' => b += 1,
//             '/' => c += 1,
//             '*' => d += 1,
//             _ => (),
//         }
//     }

//     (a, b, c, d)
// }
