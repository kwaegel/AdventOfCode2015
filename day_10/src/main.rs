

fn look_and_say(input: &str) -> String {

    let mut output = "".to_string();
    let mut chr_count = 0;
    let mut last_chr = ' ';
    for chr in input.chars() {
        if chr == last_chr {
            chr_count += 1;
        } else if chr_count > 0 {
            let substr = format!("{}{}", chr_count, last_chr);
            output.push_str(&substr);
            chr_count = 1;
        } else {
            chr_count = 1;
        }

        last_chr = chr;
    }

    if chr_count > 0 {
        let substr = format!("{}{}", chr_count, last_chr);
        output.push_str(&substr);
    }
    // println!("{} => {}", input, output);
    output
}

fn main() {

    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");

    let input = "1113222113";

    let mut stage = input.to_string();
    for _ in 0..40 {
        stage = look_and_say(&stage);
    }

    // println!("Final string: {}", stage);
    println!("Part 1: final string is {} characters long", stage.len());

    stage = input.to_string();
    for _ in 0..50 {
        stage = look_and_say(&stage);
    }

    println!("Part 2: final string is now {} characters long",
             stage.len());
}
