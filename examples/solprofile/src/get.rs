use anchor_lang::prelude::*;

use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Signer},
    Client, Cluster,
};

use crate::util::{get_default_signer_keypair_path, get_profile_account};
use std::rc::Rc;

declare_program!(solprofile);

mod util;

fn main() {
    let program_id = solprofile::ID;

    let signer_keypair_path = get_default_signer_keypair_path();
    let signer = read_keypair_file(signer_keypair_path).unwrap();
    let signer_key = Signer::pubkey(&signer);

    let client = Client::new(Cluster::Localnet, Rc::new(&signer));
    let program = client.program(program_id).unwrap();

    let (profile_key, _) = get_profile_account(&signer_key, &program_id);

    let data: solprofile::accounts::Profile = program.account(profile_key).unwrap();

    println!("data: {data:#?}");
}
