use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("FvfiKfUPLZvrAJnmeAZtMZi51FvYuoWr9WvozxDrb6aE");

#[program]
pub mod solidea {
    use super::*;
    pub fn post_job(ctx: Context<PostJob>, jobtype: String, content: String) -> Result<()> {
        let job: &mut Account<Job> = &mut ctx.accounts.job;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if jobtype.chars().count() > 50 {
            return Err(ErrorCode::JobtypeTooLong.into())
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        job.author = *author.key;
        job.timestamp = clock.unix_timestamp;
        job.jobtype = jobtype;
        job.content = content;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PostJob<'info> {
    #[account(init, payer = author, space = JobPost::LEN)]
    pub job: Account<'info, Job>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

// #[account]
// pub struct User {
//     pub author: Pubkey,
//     pub userId: i64,
//     pub name: String,
//     pub emailId: String,
// }

#[account]
pub struct Job {
    pub author: Pubkey,
    pub timestamp: i64,
    pub jobtype: String,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32; //[u8,32]
const TIMESTAMP_LENGTH: usize = 8;
const STRING_SIZE_PREFIX: usize = 4;
const MAX_JOB_TYPE_SIZE: usize = 50 * 4;
const MAX_CONTENT_SIZE: usize = 280 * 4;

impl Job {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + STRING_SIZE_PREFIX + MAX_JOB_TYPE_SIZE
        + STRING_SIZE_PREFIX + MAX_CONTENT_SIZE;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Max jobtype should be 50 characters long.")]
    JobtypeTooLong,
    #[msg("Max content should be 280 characters long.")]
    ContentTooLong,
}