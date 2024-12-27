use anchor_lang::prelude::*;

declare_id!("q3z1zycbc3BZrKbTJ49C7nJVYtAo61rUNavCtZUjdDU");

#[program]
pub mod solana_interview {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>, id:u64, title:String, description:String) -> Result<()> {
        msg!("Task created {:?}", id);
        let task  =  &mut ctx.accounts.task_account;
        task.id =  id;
        task.title = title;
        task.description =  description;
        task.completed =  false;
        task.owner = ctx.accounts.owner.key();
        let clock =  Clock::get().unwrap();
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.bump= ctx.bumps.task_account;
        Ok(())
    }
    pub fn update_task(ctx: Context<UpdateTask>, id:u64, title:String, description:String,task_bump:u8) -> Result<()> {
        msg!("Task updated {:?}", id);

        let task  =  &mut ctx.accounts.task_account;
        task.title = title;
        task.description =  description;
        let clock =  Clock::get().unwrap();
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>,id:u64, task_bump:u8) -> Result<()> {
        msg!("Task completed {:?}", id);
        let task  =  &mut ctx.accounts.task_account;
        task.completed =  true;
        let clock =  Clock::get().unwrap();
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct CreateTask<'info> {
    #[account(
        init, 
        space = 8 + 8 + 4 + 200 + 4 + 800 + 1 + 32 + 8 + 8, 
        payer = owner,
        seeds =  [b"task", owner.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    task_account:Account<'info, Task>, 
    #[account(mut)]
    owner: Signer<'info>,
    system_program: Program<'info, System>, 
    rent: Sysvar<'info,Rent>

}

#[derive(Accounts)]
#[instruction(id:u64, task_bump:u8)]
pub struct UpdateTask<'info> {
    #[account(
        mut,
        seeds = [ b"task", owner.key().as_ref(), &id.to_le_bytes()],
        bump= task_bump,
        has_one = owner
    )]
    task_account:Account<'info, Task>, 
    #[account(
        mut,
        constraint = owner.key() == task_account.owner
    )]
    owner: Signer<'info>,
    system_program: Program<'info, System>, 
    rent: Sysvar<'info,Rent>
}

#[derive(Accounts)]
#[instruction(id:u64, task_bump:u8)]
pub struct CompleteTask<'info> {
    #[account(
        mut,
        seeds =  [b"task", owner.key().as_ref(), &id.to_le_bytes()],
        bump= task_bump,
        has_one = owner
    )]
    task_account:Account<'info, Task>, 
    #[account(
        mut,
        constraint = owner.key() == task_account.owner
    )]
    owner: Signer<'info>,
    system_program: Program<'info, System>, 
    rent: Sysvar<'info,Rent>
}

#[account]
#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,          // Max 50 characters
    pub description: String,    // Max 200 characters
    pub completed: bool,
    pub owner: Pubkey,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8
}