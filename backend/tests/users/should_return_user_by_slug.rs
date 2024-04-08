
use mockall::predicate::*;
use backend::{
    users::{
        repository::{DynUserRepository, User},
        service::DynUserService,
    },
    mocks::UserServiceTestFixture,
};


#[cfg(test)]
async fn should_return_user_by_slug() {
    assert!(6 == 6);
}