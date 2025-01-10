mod programs;
#[cfg(test)]
mod tests {
    const RPC_URL: &str = "https://api.devnet.solana.com";
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        signature::{read_keypair_file, Keypair, Signer}, system_program,
        transaction::Transaction,
    };
    #[test]
    fn interact() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {
            github: b"Juiiceee".to_vec()
        };
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[
                &signer.pubkey(),
                &prereq,
                &system_program::id()
            ],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
}
