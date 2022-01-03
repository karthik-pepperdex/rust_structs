// creating a struct. a struct's first letter by convention is capitalized 
struct User {
    // a struct is a type containing sub-types
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn structs() {
    // way 1 of calling a struct
    let user1 = User {
        email: String::from("johndoe@gmail.com"),
        username: String::from("johndoe"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username;
    let email = user1.email;
    let sign_in_count = user1.sign_in_count;
    let active = user1.active;
    println!("Way 1 of calling stucts: {} {} {} {}", name, email, sign_in_count, active);

    // way 2 of calling a struct
    let user_2 = user_2(String::from("johndoe@gmail.com"), String::from("johndoe"), true, 2);
    let name1 = user_2.username;
    let email1 = user_2.email;
    let sign_in_count1 = user_2.sign_in_count;
    let active1 = user_2.active;
    println!("Way 2 of calling structs: {} {} {} {}", name1, email1, sign_in_count1, active1);

    // reusing the provided data of a particular instance
    let user_3 = User {
        username: String::from("johndoe"),
        email: String::from("johndoe@gmail.com"),
        ..user1
    };
    let name2 = user_3.username;
    let email2 = user_3.email;
    let sign_in_count2 = user_3.sign_in_count;
    let active2 = user_3.active;
    println!("Way 3 of calling structs: {} {} {} {}", name2, email2, sign_in_count2, active2);
}

fn user_2(user_email: String, user_name: String, user_is_active: bool, user_sign_in_count: u64) -> User { // because a struct is a type containing sub-types we can use the struct as the return type
    User {
        email: user_email,
        username: user_name,
        active: user_is_active,
        sign_in_count: user_sign_in_count,
    }
}