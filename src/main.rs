use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::{thread, time::Duration, str::FromStr, io::{self, Write, Error as IoError, ErrorKind}};
use borsh::{BorshDeserialize, BorshSerialize};

// Define U256 using the uint crate
uint::construct_uint! {
    pub struct U256(4);
}

// Custom to_le_bytes implementation for U256
impl U256 {
    pub fn to_le_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for (index, word) in self.0.iter().enumerate() {
            bytes[index * 8..(index + 1) * 8].copy_from_slice(&word.to_le_bytes());
        }
        bytes
    }
}

// Manually implement BorshDeserialize and BorshSerialize for U256
impl BorshSerialize for U256 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), IoError> {
        let bytes = self.to_le_bytes();
        writer.write_all(&bytes)
    }
}

impl BorshDeserialize for U256 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, IoError> {
        const SIZE: usize = 32; // U256 expected size in bytes
        if buf.len() < SIZE {
            return Err(IoError::new(ErrorKind::Other, "Buffer too short to deserialize U256"));
        }
        let mut array = [0u8; SIZE];
        array.copy_from_slice(&buf[0..SIZE]);
        *buf = &buf[SIZE..]; // Move the slice forward
        Ok(U256::from_little_endian(&array))
    }
}


#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct PoolState {
    token_a_balance: U256,
    token_b_balance: U256,
}

fn deserialize_pool_state(data: &[u8]) -> Result<PoolState, IoError> {
    println!("Attempting to deserialize data of length: {}", data.len());
    PoolState::try_from_slice(data)
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

    loop {
        for address in &pool_addresses {
            let pool_pubkey = Pubkey::from_str(address).unwrap();
            match client.get_account_data(&pool_pubkey) {
                Ok(data) => {
                    match deserialize_pool_state(&data) {
                        Ok(pool_state) => println!("Data for Pool {}: {:?}", address, pool_state),
                        Err(e) => println!("Error processing data for {}: {:?}", address, e),
                    }
                },
                Err(e) => println!("Failed to fetch data for {}: {:?}", address, e),
            }
        }

        thread::sleep(Duration::from_secs(10)); // Adjust the frequency as necessary
    }
}

