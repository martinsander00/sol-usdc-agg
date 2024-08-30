use std::time::Instant;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::{thread, time::Duration, str::FromStr, io::{self, Read}};
use borsh::{BorshDeserialize, BorshSerialize};
use termion::{clear, cursor, color};
use std::io::Write;
use std::collections::HashMap;
// Define U128 using the uint crate
uint::construct_uint! {
    pub struct U128(2);
}

// Implement to_le_bytes and from_le_bytes for U128
impl U128 {
    fn to_le_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        self.to_little_endian(&mut bytes);
        bytes
    }

    fn from_le_bytes(bytes: [u8; 16]) -> Self {
        U128::from_little_endian(&bytes)
    }
}

// Implement BorshSerialize and BorshDeserialize for U128
impl BorshSerialize for U128 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let bytes = self.to_le_bytes();
        writer.write_all(&bytes)
    }
}

impl BorshDeserialize for U128 {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let mut bytes = [0u8; 16];
        buf.read_exact(&mut bytes)?;
        Ok(U128::from_le_bytes(bytes))
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Default, Clone, Copy)]
struct WhirlpoolRewardInfo {
    mint: Pubkey,
    vault: Pubkey,
    authority: Pubkey,
    emissions_per_second_x64: U128,
    growth_global_x64: U128,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct Whirlpool {
    whirlpools_config: Pubkey,
    whirlpool_bump: [u8; 1],
    tick_spacing: u16,
    tick_spacing_seed: [u8; 2],
    fee_rate: u16,
    protocol_fee_rate: u16,
    liquidity: U128,
    sqrt_price: U128,
    tick_current_index: i32,
    protocol_fee_owed_a: u64,
    protocol_fee_owed_b: u64,
    token_mint_a: Pubkey,
    token_vault_a: Pubkey,
    fee_growth_global_a: U128,
    token_mint_b: Pubkey,
    token_vault_b: Pubkey,
    fee_growth_global_b: U128,
    reward_last_updated_timestamp: u64,
    reward_infos: [WhirlpoolRewardInfo; 3],
}

fn deserialize_whirlpool(data: &[u8]) -> Result<Whirlpool, io::Error> {
    let mut data = &data[8..]; // Skip the 8-byte discriminator
    Whirlpool::deserialize(&mut data)
}

fn main() {
    let rpc_url = String::from("https://api.mainnet-beta.solana.com");
    let client = RpcClient::new(rpc_url);

    let pool_addresses = vec![
        "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE",
        "FpCMFDFGYotvufJ7HrFHsWEiiQCGbkLCtwHiDnh7o28Q",
        "7qbRF6YsyGuLUVs6Y1q64bdVrfe4ZcUUz1JRdoVNUJnm",
        "83v8iPyZihDEjDdY8RdZddyZNyUtXngz69Lgo9Kt5d6d",
        "HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ",
        "21gTfxAnhUDjJGZJDkTXctGFKT8TeiXx6pN1CEg9K1uW",
        "DFVTutNYXD8z4T5cRdgpso1G3sZqQvMHWpW2N99E4DvE",
        "7xuPLn8Bun4ZGHeD95xYLnPKReKtSe7zfVRzRJWJZVZW",
        "6d4UYGAEs4Akq6py8Vb3Qv5PvMkecPLS1Z9bBCcip2R7",
        "CWjGo5jkduSW5LN5rxgiQ18vGnJJEKWPCXkpJGxKSQTH"
    ];

    // HashMap to store the previous prices
    let mut last_prices: HashMap<String, f64> = HashMap::new();
    let mut first_iteration = true;

    loop {
        if first_iteration {
            // Clear the terminal screen on the first iteration
            print!("{}", clear::All);
            print!("{}", cursor::Goto(1, 1));
            first_iteration = false;
        }

        let mut updated = false;

        for address in &pool_addresses {
            let pool_pubkey = Pubkey::from_str(address).unwrap();
            match client.get_account_data(&pool_pubkey) {
                Ok(data) => {
                    match deserialize_whirlpool(&data) {
                        Ok(whirlpool) => {
                            let sqrt_price = whirlpool.sqrt_price.as_u128();

                            // Convert sqrt_price from Q64.64 format to f64
                            let sqrt_price_f64 = (sqrt_price as f64) / (1u128 << 64) as f64;

                            // Square the sqrt_price to get the actual price
                            let price = sqrt_price_f64 * sqrt_price_f64;

                            // Adjust price by moving the decimal 3 places to the right
                            let adjusted_price = price * 1000.0;

                            // Check if the price has changed
                            let address_str = format!("{}...{}", &address[0..5], &address[address.len()-5..]);
                            if last_prices.get(&address_str) != Some(&adjusted_price) {
                                // Print only the first and last 5 characters of the address and the price
                                print!(
                                    "{}{} - ${:.6}\n",
                                    cursor::Goto(1, pool_addresses.iter().position(|x| x == address).unwrap() as u16 + 1),
                                    address_str,
                                    adjusted_price
                                );

                                // Update the HashMap with the new price
                                last_prices.insert(address_str, adjusted_price);
                                updated = true;
                            }
                        },
                        Err(e) => println!("{} - Error processing data: {:?}", address, e),
                    }
                },
                Err(e) => println!("{} - Failed to fetch data: {:?}", address, e),
            }
        }

        if updated {
            // Force flush the output to ensure it updates in the terminal
            io::stdout().flush().unwrap();
        }

        // Sleep before the next iteration
        thread::sleep(Duration::from_secs(2));
    }
}
