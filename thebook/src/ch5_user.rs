pub struct User {
    pub name: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: u64,
}

pub fn create(name: String, email: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

pub fn clone(user: &User) -> User {
    User {
        name: user.name.clone(),
        email: user.email.clone(),
        ..*user
    }
}

pub fn create2(name: &str, email: &str) -> User {
    create(String::from(name), String::from(email))
}

pub fn to_string(user: &User) -> String {
    format!(
        "{{name: '{}', email: '{}', active: '{}', sign_in_count: '{}'}}",
        user.name, user.email, user.active, user.sign_in_count
    )
}
