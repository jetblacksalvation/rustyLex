
use std::fs;
use std::env;

#[allow(non_snake_case)]
#[allow(unused_must_use)]

static  SEPERATORS: &'static str = "()[]{}/=+-,.";

fn parse_into_tokens(contents : String) -> Vec<String>{
    let mut tokens: Vec<String> = vec![];
    let mut strBuffer: String = String::from("");
    for character in contents.chars(){
        for sep in SEPERATORS.chars(){
            if sep == character{
                tokens.push(strBuffer.clone());
                tokens.push(sep.to_string());
                strBuffer.clear();
            }
        }
        if character.is_alphanumeric() {
            strBuffer += &character.to_string();

        }
    }
    return tokens;
}

fn main() {
    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);
    let args : Vec<String>= env::args().collect();
    let mut errorMessage:String;

    match args.len(){
        1 => 
        {
            let path = args[1].clone();
            errorMessage = format!("unable to read {}", path);
            let contents = fs::read_to_string(path)
            .expect(&errorMessage);
            let tokens = parse_into_tokens(contents);
            for token in tokens{
                print!("{}",token);
            }
            errorMessage.clear();
        },
        2 =>
        {
            let path = args[1].clone();

            errorMessage = format!("unable to read {}", path);
            let contents = fs::read_to_string(path)
        .expect(&errorMessage);
            let tokens = parse_into_tokens(contents);
            for token in tokens{
                print!("{}, ",token);
            }
            errorMessage.clear();
        },
        _ => {print!("Unexpected arg count! arg count = {:?}", args.len())}
    }

    println!("Hello, world!");
}
