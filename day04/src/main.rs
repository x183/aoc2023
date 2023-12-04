use std::fs;
fn p1() {
    let contents = fs::read_to_string("realData").expect("Something went wrong reading the file");
    let lines = contents.lines().map(|x| x.split(": ").nth(1).unwrap().split("|"));
    let mut total_score = 0;
    for mut line in lines{
        let winning_nums = line.next().unwrap().split(" ");
        let had_nums = line.next().unwrap().split(" ");
        let mut card_score = 0;
        for num in winning_nums {
            let had_copy = had_nums.clone();
            if had_copy.fold(false, |previous, x| x == num && x!="" || previous){
                print!("{} ", num);
                if card_score != 0 {
                    card_score <<= 1;
                }else {
                    card_score = 1;
                }
            }
        }
        println!("\n{}", card_score);
        total_score += card_score;

    }
    println!("{}", total_score);
}
fn p2(){
    let contents = fs::read_to_string("exInput").expect("Something went wrong reading the file");
    let lines = contents.lines().map(|x| x.split(": ").nth(1).unwrap().split("|"));
    let mut total_score =[0;200];
    for line_index  in 0..lines.clone().count(){
        let mut line = lines.clone().nth(line_index).unwrap();
        let winning_nums = line.next().unwrap().split(" ");
        let had_nums = line.next().unwrap().split(" ");
        let mut card_score = 0;
        for num in winning_nums {
            let had_copy = had_nums.clone();
            if had_copy.fold(false, |previous, x| x == num && x!="" || previous){
                //print!("{} ", num);
                card_score += 1;
            }
        }
        total_score[line_index] = card_score;
        println!{"{}", total_score[line_index]};
    }
    let mut final_score = [0;200];
    for
}

fn main() {
    p1();
    p2();
}
