use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair_file},
    Client, Cluster,
};

#[test]
fn test_initialize() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = solprofile::id();
    let program = client.program(program_id).unwrap();

    let tx = program
        .request()
        .accounts(solprofile::accounts::Initialize {})
        .args(solprofile::instruction::Initialize {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
