#[derive(Clone)]
pub struct User {
    name: String,
    age: usize,
}

impl User {
    pub fn new(name: &str, age: usize) -> User {
        let name = String::from(name);
        User { name, age }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User ( name: {}, age: {} )", self.name, self.age)
    }
}

#[derive(Clone)]
pub struct UserRepository {
    users: Vec<User>,
}

pub static mut USER_REPO_INSTANCE: UserRepository = UserRepository::new();

impl UserRepository {
    const fn new() -> UserRepository {
        UserRepository { users: vec![] }
    }

    pub fn add(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn remove(&mut self, index: usize) {
        self.users.remove(index);
    }

    pub fn show(&self) {
        for user in self.users.iter() {
            println!("{}", user);
        }
    }
}
