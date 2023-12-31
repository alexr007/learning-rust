use std::io::{BufWriter, Stdin, stdout};
use std::num::ParseIntError;
use std::ops::Add;
use std::str::{Bytes, Chars};
use ferris_says::say;

pub fn playground() {
    /// string in the stack
    let s1: &str = "abc";
    /// heap
    let s2: String = String::from("hello");

    /// print macro
    println!("{}", s1);
    /// print macro
    println!("{s1}");
    /// debug
    dbg!(s2);

    /// mutable !
    let mut s3 = String::from("hello");
    s3.push_str(", world!");
    dbg!(s3);

    let s = String::from("hello world");
    /// slicing doesn't take a memory
    let hello = &s[0..5];
    let world = &s[6..11];
    let s1 = &s[..5];
    let s2 = &s[6..];

    // let stdout = std::io::stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(&message, width, &mut writer).unwrap();

    let _ = "  Hello  ".trim(); // "Hello"

    /// parsing
    let r: Result<u32, ParseIntError> = "5".parse();
    /// handle explicitly
    let int: u32 = r.clone().expect("should be a number");
    /// allow to panic
    let r: u32 = r.unwrap();

    /// stack
    let data = "initial contents";
    /// move to heap
    let s = data.to_string();
    let s = "initial contents".to_string();
    /// initially in heap
    let s = String::from("initial contents");
    /// UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::new();
    /// grow
    s.push_str("abc");
    s.push_str("def");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /// deref
    let s2a = &s2[..];
    /// fn +(self, s: &str) -> String
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    let chars = &hello[..];
    // string access is not O(1) because we need to traverse it from start
    let xs = "नमस्ते"; // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //                       ['न', 'म', 'स', '्', 'त', 'े']
    //                       ["न", "म", "स्",      "ते"     ]

    /// A String is a wrapper over a Vec<u8>
    let s1 = String::from("hello");
    // let s1 = "hello";
    // let h = s1[0]; // `String` cannot be indexed by `{integer}`
    println!("{}", s1.len()); // 5
    let hello = "Здравствуйте";
    println!("{}", hello.len()); // 24
    /// char is always 4 bytes
    let chars: Chars = hello.chars();
    for c in chars {
        dbg!(c);
    }
    let chars: Bytes = hello.bytes();
    for c in chars {
        dbg!(c);
    }
    //dbg!("{}", chars.count());
    // chars.for_each(|x|println!("{}", x));

    let hello = "Здравствуйте";
    dbg!(&hello[0..4]); // byte slice
    dbg!(&hello[0..6]); // byte slice
    //dbg!(&hello[0..5]); // panic ! half-char
}
