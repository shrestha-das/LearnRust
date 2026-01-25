#![allow(unused)]

// Question Operator - ?

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(2)
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

// Method: 1 === Using Match
fn f1_f2_match() -> Result<u32, String> {
    let res_1 = f1();
    let out_1 = match res_1 {
        Ok(num) => num,
        Err(_) => {
            return Err("error from f2".to_string());
        }
    };
    let res_2 = f2();
    let out_2 = match res_2 {
        Ok(num) => num,
        Err(_) => {
            return Err("error from f2".to_string());
        }
    };
    Ok(out_1 + out_2)
}

// Method: 2 === Use ?
fn f1_f2_question() -> Result<u32, String> {
    let out_1 = f1()?;
    let out_2 = f2()?;
    Ok(out_1 + out_2)
}

fn main() {
    let res1 = f1_f2_match();
    println!("result of f1_f2_match = {:?}", res1);
    let res2 = f1_f2_question();
    println!("Result of f1_f2_question = {:?}", res2);
      
}
