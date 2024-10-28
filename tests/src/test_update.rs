use anchor_client::{
    solana_client::client_error::ClientErrorKind,
    solana_client::rpc_request::RpcError,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Signer},
        system_program,
    },
    Client, ClientError, Cluster,
};

#[test]
fn test_update_ok() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let signer = read_keypair_file(&anchor_wallet).unwrap();
    let signer_key = Signer::pubkey(&signer);

    let client =
        Client::new_with_options(Cluster::Localnet, &signer, CommitmentConfig::confirmed());
    let program_id = solprofile::id();
    let program = client.program(program_id).unwrap();
    let system_program_key = system_program::id();

    let (profile_key, _) = solprofile::instructions::get_profile_account(&signer_key, &program_id);

    {
        program
            .request()
            .accounts(solprofile::accounts::Create {
                profile: profile_key,
                owner: signer_key,
                system_program: system_program_key,
            })
            .args(solprofile::instruction::Create)
            .signer(&signer)
            .send()
            .expect("Failed to create profile");

        {
            let data: solprofile::states::Profile = program
                .account(profile_key)
                .expect("Failed to fetch account");
            assert_eq!(data.name, "");
        }
    }

    let expected_name = "update";
    program
        .request()
        .accounts(solprofile::accounts::Update {
            profile: profile_key,
            owner: signer_key,
            system_program: system_program_key,
        })
        .args(solprofile::instruction::Update {
            args: solprofile::instructions::UpdateArgs {
                name: Some(expected_name.to_string()),
            },
        })
        .signer(&signer)
        .send()
        .expect("Failed to update profile");

    {
        let data: solprofile::states::Profile = program
            .account(profile_key)
            .expect("Failed to fetch account");
        assert_eq!(data.name, expected_name);
    }
}

#[test]
fn test_update_ng_with_profile_not_created() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let signer = read_keypair_file(&anchor_wallet).unwrap();
    let signer_key = Signer::pubkey(&signer);

    let client =
        Client::new_with_options(Cluster::Localnet, &signer, CommitmentConfig::confirmed());
    let program_id = solprofile::id();
    let program = client.program(program_id).unwrap();
    let system_program_key = system_program::id();

    let (profile_key, _) = solprofile::instructions::get_profile_account(&signer_key, &program_id);

    let expected_error_code = format!("0x{:x}", 3012); // AccountNotInitialized
    assert_eq!(expected_error_code, "0xbc4");

    let result = program
        .request()
        .accounts(solprofile::accounts::Update {
            profile: profile_key,
            owner: signer_key,
            system_program: system_program_key,
        })
        .args(solprofile::instruction::Update {
            args: solprofile::instructions::UpdateArgs {
                name: Some("update".to_string()),
            },
        })
        .signer(&signer)
        .send();

    match result {
        Ok(_) => assert!(false, "Expected error"),
        Err(err) => match err {
            ClientError::SolanaClientError(err) => match err.kind() {
                ClientErrorKind::RpcError(err) => match err {
                    RpcError::RpcResponseError { message, .. } => {
                        assert!(
                            message.contains(&expected_error_code),
                            "Expected message to contain expected_error_code"
                        )
                    }
                    _ => assert!(false, "Expected RpcError::RpcResponseError"),
                },
                _ => assert!(false, "Expected ErrorKind::RpcError"),
            },
            _ => assert!(false, "Expected ClientError::SolanaClientError"),
        },
    }
}
