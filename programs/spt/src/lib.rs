use anchor_lang::prelude::*;

declare_id!("DkeDgUpXLMQNk4VLJYEgEkvMkj58UTwJghTJFNinufm3");

#[program]
pub mod spt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[cfg(test)]
mod tests {
    use anchor_lang::{
        prelude::*,
        solana_program::{entrypoint::ProgramResult, instruction::Instruction},
        InstructionData,
    };
    use solana_program_test::{processor, ProgramTest};
    use solana_sdk::{signer::Signer, transaction::Transaction};

    // here's the magic sauce
    fn my_process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        crate::entry(program_id, unsafe { std::mem::transmute(accounts) }, data)
    }

    #[tokio::test]
    async fn test_spt() {
        let pt = ProgramTest::new("spt", crate::ID, processor!(my_process));
        let mut ctx = pt.start_with_context().await;

        let instruction = Instruction {
            program_id: crate::ID,
            accounts: crate::accounts::Initialize {}.to_account_metas(None),
            data: crate::instruction::Initialize.data(),
        };
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer],
            ctx.last_blockhash,
        );

        ctx.banks_client
            .process_transaction(transaction)
            .await
            .unwrap();
    }
}
