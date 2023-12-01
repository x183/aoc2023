use std::fs;

/* fn main() {
    let file = fs::read_to_string("ex1Data").expect("UwU");
    let lines = file.split("\n");
    let mut sum = 0;
    for line in lines {
        let l = line.chars();
        let mut n1=' '; let mut n2 = ' ';
        for c in l {
            if c.is_ascii_digit() {
                if n1 == ' ' {n1 = c;}
                /* if n2 == 'a' {break; } */
                n2 = c;
            }
        }
        let checksum:i32 = (n1 as i32 - 48)*10 + (n2 as i32 - 48);
        if n1 == ' ' {continue;}
        println!("{checksum}");
        sum += checksum;
        //sum += "{n1}{n2}".to_string().parse::<i32>().unwrap();

    }
    println!("Data p1:\n\t{sum}");
} */

fn main() {
    let file = fs::read_to_string("ex1Data").expect("UwU");
    let lines = file.split("\n");
    let mut sum = 0;
    let nums = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    for line in lines {
        let mut n1=0; let mut n2 = 0;

        for i in 0..(line.len()) {
            let c:char = line.chars().nth(i).expect("OwO");

            if c.is_ascii_digit() {
                if n1 == 0 {n1 = c as i32 - 48;}
                /* if n2 == 'a' {break; } */
                n2 = c as i32 - 48;
            }/* else if (){} */else {
                for n in 0..(nums.len()){

                    if nums[n].chars().nth(0).expect("Waa") == c{
                        let mut passed = true;
                        for o in 0..(nums[n].len()){
                            if o+i >= line.len() {
                                passed=false;
                                break;
                            }
                            if nums[n].chars().nth(o).expect("wooo") != line.chars().nth(o+i).expect("wee"){

                                passed = false;
                                break;
                            }
                        }
                        if passed{
                            if n1 == 0 {n1 = n as i32;}
                            n2 = n as i32;
                        }

                    }
                }
            }
        }
        let checksum:i32 = (n1 )*10 + (n2 );
        if n1 == 0 {continue;}
        println!("{checksum}");
        sum += checksum;
        //sum += "{n1}{n2}".to_string().parse::<i32>().unwrap();

    }
    println!("Data p1:\n\t{sum}");
}