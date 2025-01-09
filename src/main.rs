use std::env;   // 환경 변수 처리 
use std::fs::File;
use std::io::prelude::*; // 읽기 제공 

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // let query = &args[1];
    // let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}",contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }




//emv::args() = args =  이터레이트 반환 ,  / 문자열로 명령 전달 <string?
// 이터레이트를 벡터로 반환  벡터 타입 문자열 전달/ 
// args 변수 저장 

//예를 들어, ./program arg1 arg2라는 명령을 실행하면, args는 ["./program", "arg1", "arg2"]가 됩니다.
// 출력 /base dir poem.txt


