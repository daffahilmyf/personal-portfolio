use crate::users::repository::MockUserRepository;

pub struct UserServiceTestFixture {
    pub mock_repository: MockUserRepository,
}

impl UserServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_repository: MockUserRepository::new(),
        }
    }
}

impl Default for UserServiceTestFixture {
    fn default() -> Self {
        UserServiceTestFixture::new()
    }
}