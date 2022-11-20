pub trait UserRepository {
    fn find(&self, user_id: String) -> ();
}
