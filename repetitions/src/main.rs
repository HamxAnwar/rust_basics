fn main() {
    // Loop
    let mut intvar = 1;
    loop {
        print!("{:?} ", intvar);
        if intvar == 4 {
            break;
        }
        intvar += 1;
    }

    println!("------------------------------");

    //While loop
    let mut intwhile = 5;
    while intwhile >=1 {
        print!("{:?} ", intwhile);
        intwhile -= 1;
    }
    println!("done!")
}