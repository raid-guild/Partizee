use std::process::Command;
use crate::utils::fs_nav::id_pbc_path;
use crate::utils::menus::force_new_wallet_menu;
use crate::utils::utils::{print_error, print_output};

pub fn pbc_create_new_account(network: &str) -> Result<(), Box<dyn std::error::Error>> {
    let network_command: String = format!("--net={}", &network);
    let output = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("create")
        .arg(&network_command)
        .output()
        .expect("Failed to create account");
    if !output.status.success() {
        return print_error(&output);
    } else {
        return print_output("pbc_create_new_account", &output);
    }
}

pub fn pbc_create_new_wallet(network: &str) -> Result<String, Box<dyn std::error::Error>> {
    let network_command: String = format!("--net={}", network);
    // check if .pbc folder exists
    if id_pbc_path().is_none() {
        // create new account
        let output = Command::new("cargo")
            .arg("pbc")
            .arg("wallet")
            .arg("create")
            .arg(&network_command)
            .output()
            .expect("Failed to create account");

        if !output.status.success() {
            return print_error(&output);
        } else {
            return print_output("pbc_create_new_wallet no force", &output);
        }
    } else if id_pbc_path().is_some() {
        // open menu to ask if user wants to create a new account
        let force_create: bool = force_new_wallet_menu().expect("Failed force new wallet menu");
        if force_create {
            let output = Command::new("cargo")
                .arg("pbc")
                .arg("wallet")
                .arg("create")
                .arg(&network_command)
                .arg("--force")
                .output()
                .expect("Failed to create account");

            if !output.status.success() {
                return print_error(&output);
            } else {
                return print_output("pbc_create_new_wallet force", &output);
            }
        } else {
            return Err("Failed to create wallet".into());
        }
    } else {
        return Err("I didn't expect to get here.  I mean, the id_pbc file exists or it doesnt.  There shouldn't be a third option.".into());
    }
}
