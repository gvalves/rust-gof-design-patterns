use singleton::*;

fn main() {
    unsafe {
        module_a();
        module_b();
        USER_REPO_INSTANCE.show();
    }
}

unsafe fn module_a() {
    USER_REPO_INSTANCE.add(User::new("Gustavo", 20));
    USER_REPO_INSTANCE.add(User::new("Maria", 25));
    USER_REPO_INSTANCE.add(User::new("Lucas", 32));
}

unsafe fn module_b() {
    USER_REPO_INSTANCE.add(User::new("Pamela", 23));
    USER_REPO_INSTANCE.add(User::new("Jill", 18));
    USER_REPO_INSTANCE.add(User::new("Leonardo", 40));
}
