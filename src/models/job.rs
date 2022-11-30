
use rocket::form::{self, Error as FormError, FromForm};
use crate::errors::our_error::OurError;

#[derive(Debug, FromForm)]
pub struct NewJob<'r> {
    #[field(validate = len(5..20).or_else(msg!("company name cannot be empty")))]
    pub company_name: &'r str,
}

#[derive(Debug)]
pub struct Job{
}

impl Job {

    pub async fn create<'r>(new_job: &'r NewJob<'r>) -> Result<Self, OurError> {
        !todo!("rin");
    }

}