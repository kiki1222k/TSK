pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

use anchor_lang::prelude::*;

// Privremeni ID - zamijenit ćemo ga nakon prvog builda
declare_id!("8urhJmRorAWojqjCwFGYjuymLqPb1PHw64JzgKBcu8cH");

#[program]
pub mod voting_dapp {
    use super::*;

    // Kreiranje nove ankete [cite: 161]
    pub fn create_poll(
        ctx: Context<CreatePoll>,
        question: String,
        options: Vec<String>,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let clock = Clock::get()?;

        poll.author = *ctx.accounts.user.key; // Autor glasanja [cite: 166]
        poll.question = question; // Naslov pitanja [cite: 164]
        poll.options = options.clone(); // Opcije glasanja [cite: 165]
        poll.votes = vec![0; options.len()]; // Inicijalizacija broja glasova [cite: 173]
        poll.voters = Vec::new(); // Lista wallet adresa glasača [cite: 171]
        poll.timestamp = clock.unix_timestamp; // Datum kreiranja [cite: 167]

        Ok(())
    }

    // Glasanje [cite: 168]
    pub fn vote(ctx: Context<Vote>, option_index: u8) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let voter = ctx.accounts.user.key();

        // Verifikacija da wallet može glasati samo jednom [cite: 48, 173]
        require!(
            !poll.voters.contains(&voter),
            ErrorCode::AlreadyVoted
        );

        // Zapisivanje odabrane opcije i povećanje broja glasova [cite: 172, 173]
        poll.votes[option_index as usize] += 1;
        poll.voters.push(voter); // Zapisivanje wallet adrese glasača [cite: 171]

        Ok(())
    }
}

// Struktura računa na blockchainu [cite: 177, 178]
#[account]
pub struct Poll {
    pub author: Pubkey,      // Autor [cite: 184]
    pub question: String,    // Naslov [cite: 181]
    pub options: Vec<String>, // Opcije [cite: 182]
    pub votes: Vec<u64>,     // Broj glasova [cite: 183]
    pub voters: Vec<Pubkey>, // Lista glasača
    pub timestamp: i64,      // Timestamp [cite: 185]
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    // Inicijalizacija računa sa dovoljno prostora (space) [cite: 179]
    #[account(init, payer = user, space = 9000)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Već ste glasali!")] // Zabrana duplog glasanja [cite: 13, 173]
    AlreadyVoted,
}

