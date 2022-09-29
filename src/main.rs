use std::io;

fn main() {
    let mut input = String::new();
    println!("Separe a expressão com espaços.\n Exemplo: 10 + 2 * 3\n");
    loop {
        io::stdin().read_line(&mut input).expect("deu merda");
        if input.contains("exit") {
            break;
        } else if input.contains("clear") {
            print!("\x1B[2J\x1B[1;1H");
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
    let mut f_is_div: bool = false;
    let mut f_is_mut: bool = false;
    println!("{:?}", nums);

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
            if op.chars().nth(i).unwrap() == '*' && f_is_div == false {
                result += nums.get(i).unwrap() * nums.get(i + 1).unwrap();
                f_is_mut = true;
            } else if op.chars().nth(i).unwrap() == '/' && f_is_mut == false {
                result += nums.get(i).unwrap() / nums.get(i + 1).unwrap();
                f_is_div = true;
            } else if op.chars().nth(i).unwrap() != '*' && f_is_mut {
                if op.chars().nth(i).unwrap() == '/' && i == op_len - 1 {
                    result /= nums.get(i + 1).unwrap();
                } else if op.chars().nth(i).unwrap() == '/' {
                    result /= nums.get(i).unwrap();
                }

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
            } else if f_is_div {
                if op.chars().nth(i).unwrap() == '*' && i == op_len - 1 {
                    if op.chars().nth(i + 1).unwrap() == '/' {
                    } else {
                        result *= nums.get(i + 1).unwrap();
                    }
                } else if op.chars().nth(i).unwrap() == '*' {
                    result *= nums.get(i).unwrap();
                }
            }
        }
    }

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

    for ch in input.split_whitespace() {
        match ch {
            "+" => op_result.push('+'),
            "-" => op_result.push('-'),
            "/" => op_result.push('/'),
            "*" => op_result.push('*'),
            _ => (),
        }
    }

    op_result
}
