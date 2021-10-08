use std::io::{self, BufRead};
use std::collections::VecDeque;

fn compute_remained_operation(operator_stack: &mut VecDeque<char>, operand_stack: &mut VecDeque<f64>) {
    while !operator_stack.is_empty() {
        let operator = operator_stack.pop_back().unwrap();
        let operand2 = operand_stack.pop_back().unwrap();
        let operand1 = operand_stack.pop_back().unwrap();
        operand_stack.push_back(match operator {
            '+' => operand1 + operand2,
            '-' => operand1 - operand2,
            '*' => operand1 * operand2,
            '/' => operand1 / operand2,
            _ => 0.0,
        })
    }
}

fn deal_operator(operator_stack: &mut VecDeque<char>, operand_stack: &mut VecDeque<f64>, current_operator: char) {
    // We need compute previous operation first, since the operand is ready
    if current_operator == '*' || current_operator == '/' {
        operator_stack.push_back(current_operator)
    } else {
        compute_remained_operation(operator_stack, operand_stack);
        operator_stack.push_back(current_operator);
    }
}

fn extract_expr_within_parentheses<'a>(current_expr: &'a str) -> (usize, &'a str) {
    let mut paired_count = 0;
    for (i, ch) in current_expr.chars().enumerate() {
        if ch == '(' {
            paired_count += 1;
        }
        if ch == ')' {
            paired_count -= 1;
        }
        if paired_count == 0 {
            return (i, &current_expr[1..i]);
        }
    }
    (0, current_expr)
}

fn compute_expression(expr: &str) -> f64 {
    let mut operator_stack: VecDeque<char> = VecDeque::new();
    let mut operand_stack: VecDeque<f64> = VecDeque::new();
    let mut current_operand = 0.0;
    let mut is_digit = false;
    let mut i = 0;
    while i < expr.len() {
        let ch = expr[i..i+1].chars().next().unwrap();
        match ch {
            '+' => {
                if is_digit {
                    operand_stack.push_back(current_operand);
                    is_digit = false;
                    current_operand = 0.0;
                }
                if !operand_stack.is_empty() {
                    deal_operator(&mut operator_stack, &mut operand_stack, ch);
                }
            },
            '-' => {
                if is_digit {
                    operand_stack.push_back(current_operand);
                    is_digit = false;
                    current_operand = 0.0;
                }
                if !operand_stack.is_empty() {
                    deal_operator(&mut operator_stack, &mut operand_stack, ch);
                } else {
                    operand_stack.push_back(-1.0);
                    operator_stack.push_back('*');
                }
            },
            '*' | '/' => {
                if is_digit {
                    operand_stack.push_back(current_operand);
                    is_digit = false;
                    current_operand = 0.0;
                }
                deal_operator(&mut operator_stack, &mut operand_stack, ch);
            },
            '(' => {
                let (matched_index, sub_expr) = extract_expr_within_parentheses(&expr[i..]);
                operand_stack.push_back(compute_expression(sub_expr));
                i += matched_index;
            },
            _ => {
                is_digit = true;
                current_operand = current_operand * 10.0 + ch.to_digit(10).unwrap() as f64
            },
        }
        i += 1;
    }
    if is_digit {
        operand_stack.push_back(current_operand);
    }
    compute_remained_operation(&mut operator_stack, &mut operand_stack);
    operand_stack.pop_back().unwrap()
}

fn main() {
    let mut input_expr = io::stdin().lock().lines().next().unwrap().unwrap();
    input_expr = input_expr.replace("[", "(");
    input_expr = input_expr.replace("]", ")");
    input_expr = input_expr.replace("{", "(");
    input_expr = input_expr.replace("}", ")");
    println!("{}", compute_expression(input_expr.as_str()));
}
