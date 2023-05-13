fn main() {
    let price: i32 = 3;
    let little_mermaid = price * 3;
    let brother_bear = price * 5;
    let hercules = price * 1;

    let total_cost = little_mermaid + brother_bear + hercules;

    println!("Rental Cost = ${}", total_cost);

    let google_pay_rate: i32 = 400;
    let amazon_pay_rate: i32 = 380;
    let facebook_pay_rate: i32 = 350;

    fn print_pay_total(rate: i32, hours: i32) {
        println!("Pay Total: ${}", rate * hours);
    }

    print_pay_total(facebook_pay_rate, 10);
    print_pay_total(google_pay_rate, 6);
    print_pay_total(amazon_pay_rate, 4);

    let class_full: bool = true;
    let schedule_conflict: bool = true;

    if class_full || schedule_conflict {
        println!("Can't Enroll");
    } else {
        println!("Just do it!");
    }

    let premium_member: bool = true;
    let expired: bool = true;
    let purchases: i32 = 4;

    fn product_offer(prm: bool, pur: i32, exp: bool) -> bool {
        if prm && !exp {
            true
        } else if pur > 2 && !exp {
            true
        } else {
            false
        }
    }

    println!("{}", product_offer(premium_member, purchases, expired));

    let username: &str = "codeup";
    let password: &str = "notastrongpassword";

    let password_size: bool = password.len() >= 5;
    println!("password test");
    println!("{}", password_size);

    let username_size: bool = username.len() <= 20;
    println!("username test");
    println!("{}", username_size);

    let not_same_check: bool = username != password;
    println!("{}", not_same_check);

    println!("White Space Check");

    fn white_space_check(user_name: &str, pass_word: &str) -> bool {
        if user_name.starts_with(' ') || user_name.ends_with(' ') || pass_word.starts_with(' ') || pass_word.ends_with(' ') {
            true
        } else {
            false
        }
    }

    println!("{}", white_space_check(username, password));
}

