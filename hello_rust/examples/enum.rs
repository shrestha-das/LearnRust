#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Pause,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 }
}

// Enum
fn main() {
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize {width:100, height:50};

    // Debug
    println!("cmd: {:?}", cmd);

    // Compare enums
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{}", cmd0 == cmd1); // For this, we needed PartialEq

    // Option enum
    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(1);
    let x: Option<i32> = None;

    // Result enum
    // Result<T, E> = Ok(T) || Error(E)
    let x: Result<i32, String> = Ok(100);
    let x: Result<i32, String> = Err(
        "Failed to parse string into number".to_string()
    );

}