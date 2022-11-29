

#[derive(Debug, FromForm)]
pub struct NewJob<'r> {
    #[field(validate = len(5..20).or_else(msg!("company name cannot be empty")))]
    pub company_name: &'r str,
}

impl Job {

}