fn print_bubble(string: &str) -> i8 {
    let mut hit: bool = false;

    print!(" ");
    for _i in string.chars() {
        print!("_");
    }
    print!("\n(");
    print!("{}", string);
    print!(")\n(");
    for (i, _c) in string.chars().enumerate() {
        if string.len() > 1 {
            if i == string.len()-2 && !hit {
                print!(" ");
                hit = true;
            } else if i == string.len()-1 && !hit {
                print!(" ");
                hit = true;
            } else{
                print!("_");
            }
        } else {
            print!(" ");
        }
    }
    print!(")\n ");
    hit = false;
    for (i, _c) in string.chars().enumerate() {
        if string.len() > 1 {
            if i == string.len()-2 && !hit {
                print!("v");
                hit = true;
            } else if i == string.len()-1 && !hit {
                print!("v");
                hit = true;
            } else {
                print!(" ");
            }
        } else {
            print!("v");
        }
    }
    print!("\n");

    return (string.chars().count()).try_into().unwrap();
}

fn print_cat(spaces: i8) {
    for _i in 3..spaces {
        print!(" ");
    }
    print!(" ^__^\n");
    for _i in 3..spaces {
        print!(" ");
    }
    print!("(,  ,)\n");
    for _i in 3..spaces {
        print!(" ");
    }
    print!(">/7Y/<\n");
    for _i in 3..spaces {
        print!(" ");
    }
    print!("(    )\n");
    for _i in 3..spaces {
        print!(" ");
    }
    print!("| __ |\n");
    for _i in 3..spaces {
        print!(" ");
    }
    print!("| || |\n");
    for _i in 3..spaces {
        print!(" ");
    }
    print!("[_][_]\n");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        print_cat(print_bubble(&args[1]));
    } else {
        println!("Please provide one argument:\n    USAGE: {} <words>\n    Preferably in quotes!", &args[0]);
    }
}
