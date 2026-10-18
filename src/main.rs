use regex::Regex;

fn main() {
    // regular expressions
    let regex_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let regex_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let regex_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    // get user input
    println!("Enter an mathematic expression: ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // calculate multiplication
    loop {
        let captures = regex_mul.captures(expression.as_str());

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();
        let math_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let multiplication = left_value * right_value;
        
        expression = expression.replace(math_expression, &multiplication.to_string());
    }

    // calculate division
    loop {
        let captures = regex_div.captures(expression.as_str());

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();
        let math_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let division = left_value / right_value;
        
        expression = expression.replace(math_expression, &division.to_string());
    }

    // calculate addition
    loop {
        let captures = regex_add.captures(expression.as_str());

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();
        let math_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let addition = left_value + right_value;
        
        expression = expression.replace(math_expression, &addition.to_string());
    }

    // calculate substraction
    loop {
        let captures = regex_sub.captures(expression.as_str());

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();
        let math_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let substraction = left_value - right_value;
        
        expression = expression.replace(math_expression, &substraction.to_string());
    }

    // show result
    println!("The result is {}", expression);
}
