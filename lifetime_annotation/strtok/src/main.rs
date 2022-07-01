fn strtok<'a, 'b>(input: &'a mut &'b str, delim: char) -> &'a str {
    if let Some(pos) = input.find(delim) {
        let result = &input[..pos];
        *input = &input[pos+delim.len_utf8()..];
        result
    } else {
        let result = *input;
        *input = "";
        result
    }
}

fn main() {
    let mut input = "";
    let mut result = "";
    {
        let data = "hello world".to_owned();
        input = data.as_str();
        result = strtok(&mut input, ' ');
        println!("{}", result);
        println!("{}", input);
    }
    println!("{}", result);
}
