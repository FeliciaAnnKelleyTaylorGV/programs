//! This example shows how to write a contrieved and basic constraint: An MFA where a signature from a different key is requiered.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::{string::{ToString, String}, vec};

use ec_core::{bindgen::Error, bindgen::*, export_program, prelude::*};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};

// TODO confirm this isn't an issue for audit
register_custom_getrandom!(always_fail);

pub struct BasicMFAProgram;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MFATransaction {
    pub message: vec::Vec<u8>,
    pub signature: Signature,
    pub signatory: String,
}

/// The list of addresses that are allowed to sign.
/// In the example below as long as one of them sign the message is valid
const SIGNATORIES: [&str; 2] = [
    "0x63f9725f107358c9115bc9d86c72dd5823e9b1e6",
    "0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97",
];

impl Program for BasicMFAProgram {
    /// This is the only function required by the program runtime. `signature_request` is an MFATransaction request
    fn evaluate(signature_request: InitialState) -> Result<(), Error> {
        let data: MFATransaction = serde_json::from_slice(&signature_request.data).map_err(|e| Error::Evaluation(e.to_string()))?;
        // To reduce an O(n) verify operation find the position of the signatory 
        let index_of_signtory = SIGNATORIES.iter().position(|&r| r == data.signatory).ok_or(Error::Evaluation("Signatory not valid".to_string()))?;
        let signatory: Address = SIGNATORIES[index_of_signtory].parse().map_err(|_| Error::Evaluation("Signatory not valid Conversion".to_string()))?;
        // verify the signature
        data.signature
            .verify(data.message, signatory)
            .map_err(|e| Error::Evaluation(e.to_string()))?;
        Ok(())
    }
}

export_program!(BasicMFAProgram);

// write a test that calls evaluate and passes it the proper parameters
#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn test_should_sign() {
        let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
            .parse::<LocalWallet>()
            .unwrap();
        let message = "signed on entropy".to_string().into_bytes();
        let signature = wallet.sign_message(message.clone()).await.unwrap();
        let mfa_transaction = MFATransaction {
            message,
            signature: signature,
            signatory: format!("0x{}", hex::encode(wallet.address()))
        };
        let signature_request = InitialState {
            data: serde_json::to_vec(&mfa_transaction).unwrap(),
        };

        assert!(BasicMFAProgram::evaluate(signature_request).is_ok());
    }

    #[actix_rt::test]
    async fn test_should_error() {
        let wallet = "dcf2cbdd171a21c480aa7f53d67f31bb102282b3ff099c78e3118b37348c72f7"
            .parse::<LocalWallet>()
            .unwrap();
        let message = "signed on entropy".to_string().into_bytes();
        let signature = wallet.sign_message(message.clone()).await.unwrap();
        let mfa_transaction = MFATransaction {
            message,
            signature: signature,
            signatory: format!("0x{}", hex::encode(wallet.address()))
        };
        let signature_request = InitialState {
            data: serde_json::to_vec(&mfa_transaction).unwrap(),
        };

        assert!(BasicMFAProgram::evaluate(signature_request).is_err());
    }
}
