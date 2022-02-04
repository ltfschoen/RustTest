// extern crate hex;
// extern crate hex_literal;
// use hex_literal::hex;

fn main() {
  let BALANCE1 = 100_000u128;
  let BALANCE2 = 200_000u128;
  let initial_accounts = vec![
      1u8,
      2u8,
  ];
  let extra_balances = vec![
    (3u8, 300_000u128),
  ];
  let a = initial_accounts
      .iter()
      .cloned()
      .map(|x| {
          // Public key (hex) of the account without the 0x prefix below
          if x == 1u8 {
              println!("initial_account treasury {:?}", x.clone());
              return (x, BALANCE2);
          } else {
              println!("initial_account other {:?}", x.clone());
              return (x, BALANCE1);
          }
      })
      // allocate extra balances
      .chain(
          extra_balances.iter()
              .map(|x|
                  (x.0.clone(), x.1.clone())
              )
      )
      .collect::<Vec<_>>();
  println!("finished with initial_accounts {:#?}", a.clone());
}
