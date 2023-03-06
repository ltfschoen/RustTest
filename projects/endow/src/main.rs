

use hex_literal::hex;
use serde_json::{Result};
use serde::{Deserialize, Serialize};
use sp_runtime::{
  traits::{
      IdentifyAccount,
      Verify,
  },
	AccountId32,
  MultiSignature,
};
use sp_core::{
  crypto::{
      UncheckedFrom,
  },
};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
struct Allocation {
    balances: Vec<(String, String)>
}

type Signature = MultiSignature;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Balance = u128;

enum Error { Invalid }

fn get_allocation(
  endowed_accounts_with_balances: Vec<(AccountId32, Balance)>
) -> Result<Vec<(AccountId32, Balance)>> {
  let mut file = File::open("/Users/ls2/code/github/ltfschoen/RustTest/projects/endow/src/balances.json").unwrap();
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap();

  let json: Allocation = serde_json::from_str(&data)?;
  let balances_json = json.balances;
  println!("balances_json {:#?}", balances_json.clone());

  let mut combined_balances: Vec<(AccountId32, Balance)> = vec![];

  if endowed_accounts_with_balances.len() != 0 {
    for e in endowed_accounts_with_balances {
      let account_public_key_endowed: String = e.0.to_string();
      println!("account_public_key_endowed {:#?}", account_public_key_endowed.clone());

      let account_balance_endowed: Balance = e.1.to_string().parse::<Balance>().unwrap();
      println!("account_balance_endowed {:#?}", account_balance_endowed.clone());

      let account_ss58_address_endowed: AccountId32 = AccountId32::from_str(&account_public_key_endowed).unwrap();
      println!("account_ss58_address_endowed {:#?}", account_ss58_address_endowed.clone());

      combined_balances.push((account_ss58_address_endowed.clone(), account_balance_endowed.clone()));
    }
  }

  if balances_json.len() != 0 {
    for e in balances_json {
      let account_public_key_json: String = e.0.to_string();
      println!("account_public_key_json {:#?}", account_public_key_json.clone());

      let account_balance_json: Balance = e.1.to_string().parse::<Balance>().unwrap();
      println!("account_balance_json {:#?}", account_balance_json.clone());

      let account_ss58_address_json: AccountId32 = AccountId32::from_str(&account_public_key_json).unwrap();
      println!("account_ss58_address_json {:#?}", account_ss58_address_json.clone());

      let index_of_match = combined_balances.clone().iter().position(|x| x.0 == account_ss58_address_json.clone());

      if let Some(idx) = index_of_match.clone() {
          println!("match b/w endowed and json for {:#?} so overwriting its balance value", account_ss58_address_json.clone());
          combined_balances[idx] = (combined_balances[idx].0.clone(), account_balance_json.clone());
      } else {
          println!("no match b/w endowed and json for {:#?} so adding it to the list", e.0.clone());
          combined_balances.push((account_ss58_address_json.clone(), account_balance_json.clone()));
      }
    }
  }

  return Ok(combined_balances);
}

const INITIAL_BALANCE: u128 = 8_750_000_000_000_000_000_000_u128;
const INITIAL_DHX: u128 = 30_000_000_000_000_000_000_000_u128;

fn main() {
  let mut endowed_accounts_with_balances: Vec<(AccountId, Balance)> = vec![];
  let endowed_accounts: Vec<AccountId> =
    vec![
      hex!["a42b7518d62a942344fec55d414f1654bf3fd325dbfa32a3c30534d5976acb21"].into(),
      hex!["106c208ac262aa3733629ad0860d0dc72d8b9152e1cdcab497949a3f9504517a"].into(),
    ];
  for x in endowed_accounts {
      if x == UncheckedFrom::unchecked_from(
          hex!("a42b7518d62a942344fec55d414f1654bf3fd325dbfa32a3c30534d5976acb21").into(),
      ) {
          endowed_accounts_with_balances.push((x, INITIAL_DHX));
      } else {
          println!("endowed_accounts_with_balances INITIAL_BALANCE {:?}", x.clone());
          endowed_accounts_with_balances.push((x, INITIAL_BALANCE));
      }
  }

  let hardspoon_balances = get_allocation(endowed_accounts_with_balances.clone()).unwrap();
  println!("hardspoon_balances {:#?}", hardspoon_balances.clone());

  let output = hardspoon_balances
    .iter()
    .cloned()
    .map(|x| {
        return (x.0.clone(), x.1.clone());
    })
    .collect::<Vec<(AccountId, Balance)>>();
    println!("output {:#?}", output.clone());
}
