extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}





// use std::env;   // 환경 변수 처리 
// use std::fs::File;
// use std::io::prelude::*; // 읽기 제공 
// // use std::process; // 프로세스 컨트롤 

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::new(&args);
//     // let config = Config::new(&args).unwrap_or_else(|err| {
//     //     println!("Problem parsing arguments: {}", err);
//     //     process::exit(1);
//     // });
//     // let query = &args[1];
//     // let filename = &args[2];

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.filename);

//     let mut f = File::open(config.filename).expect("file not found");

//     let mut contents = String::new();
//     f.read_to_string(&mut contents).expect("something went wrong reading the file");

//     println!("With text:\n{}",contents);
// }

// struct Config {
//     query: String,
//     filename: String,
// }
// impl Config {
//     fn new(args: &[String]) -> Config {
//             if args.len() < 3 {
//         panic!("not enough arguments");
//     } // len 3 이상이면 에러 메시지 삽입 
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Config { query, filename }
        
//     }
//  }  //패닉으로 강제 프로그램 종료 

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }   // config 에 ok 반환값을줌  장기적 유지에 도움됨 


// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();    
//     let filename = args[2].clone();

//     Config { query, filename }
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];    
//     let filename = &args[2];

//     (query, filename)
// }

// fn main() {
//     // ...snip...

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.filename);

//     run(config);
// }

// fn run(config: Config) {
//     let mut f = File::open(config.filename).expect("file not found");

//     let mut contents = String::new();
//     f.read_to_string(&mut contents).expect("something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }




//   함수 비교  
//1. 구조체 반환 : 데이터 구조화, 독립적 데이터 전달 , 코드 가독성 좋음, 확장 좋음 (단점 김)
// 2. 튜플 반환 : 간단한 데이터 전달 / 메모리 효율좋음 : 반환 수명: 호출자 = 데이터
// 3. 임펠 콘픽: 표준 생성자/ 구조화 설계 / 추천되는 설계 


//emv::args() = args =  이터레이트 반환 ,  / 문자열로 명령 전달 <string?
// 이터레이트를 벡터로 반환  벡터 타입 문자열 전달/ 
// args 변수 저장 

//예를 들어, ./program arg1 arg2라는 명령을 실행하면, args는 ["./program", "arg1", "arg2"]가 됩니다.
// 출력 /base dir poem.txt





