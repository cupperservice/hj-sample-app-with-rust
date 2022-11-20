use crate::domains::repository::user_repository::UserRepository;

pub struct UserRepositoryImpl {
}

impl UserRepositoryImpl {
    pub fn new() -> UserRepositoryImpl {
        UserRepositoryImpl {}
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find(&self, user_id: String) -> () {
        todo!()
    }
}
