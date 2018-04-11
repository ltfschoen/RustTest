// External Crates
extern crate ethereum_types;
extern crate tiny_keccak;
extern crate indextree;

pub struct Header {
    shard_id: ethereum_types::U256,
    proposer_address: ethereum_types::Address,
    chunk_root: ethereum_types::H256,
    period: ethereum_types::U256,
    // The following fields are pending updates to the sharding spec and are currently ignored
    //parent_hash: ethereum_types::H256,
    //proposer_bid: ethereum_types::U256,
    //proposer_signature: ethereum_types::Signature
}

impl Header {
    pub fn new(shard_id: ethereum_types::U256, 
               //parent_hash: ethereum_types::H256,
               chunk_root: ethereum_types::H256,
               period: ethereum_types::U256,
               proposer_address: ethereum_types::Address,
               //proposer_bid: ethereum_types::U256,
               /*proposer_signature: ethereum_types::Signature*/
        ) -> Header {
        
        Header {
            shard_id,
            //parent_hash,
            chunk_root,
            period,
            proposer_address,
            //proposer_bid,
            //proposer_signature
        }
    }

    pub fn hash(&self) -> ethereum_types::H256 {
        let mut sha3 = tiny_keccak::Keccak::new_sha3_256();
        
        // Add the shard id
        let sid: &mut [u8; 32] = &mut [0; 32]; 
        u256_to_bytes32(self.shard_id, sid);
        sha3.update(sid);

        // Add the parent hash
        /*
        let ph: &mut [u8; 32] = &mut [0; 32];
        self.parent_hash.copy_to(ph);
        sha3.update(ph);
        */

        // Add the chunk root
        let cr: &mut [u8; 32] = &mut [0; 32];
        self.chunk_root.copy_to(cr);
        sha3.update(cr);

        // Add the period
        let p: &mut [u8; 32] = &mut [0; 32];
        u256_to_bytes32(self.period, p);
        sha3.update(p);

        // Add the proposer address
        let pa: &mut [u8; 20] = &mut [0; 20];
        self.proposer_address.copy_to(pa);
        sha3.update(pa);

        // Finalize hash and return as H256
        let mut result_bytes: [u8; 32] = [0; 32];
        sha3.finalize(&mut result_bytes);

        ethereum_types::H256::from_slice(&result_bytes[..])
    }
}

// Converting ethereum_types::U256 to a u8 byte array to be hashed
fn u256_to_bytes32(u256: ethereum_types::U256, dst: &mut [u8; 32]) {
    for i in 0..32 {
        dst[i] = u256.byte(i);
    }
}

pub fn collator_example() {
    // COLLATION HEADER
    
    // Create hash instance of message digest algorithm Keccak256.
    // Message digest algorithms represent the functionality of 
    // an one-way hash function for computing a fixed sized 256-bit
    // hash-value of data (message digest, hash) from input data 
    // of arbitrary size.
    let mut sha3 = tiny_keccak::Keccak::new_sha3_256();
    
    // Add the Shard ID
    let shard_id_bytes: &mut [u8; 32] = &mut [0; 32];
    // Convert from decimal string
    let shard_id = ethereum_types::U256::from_dec_str("1").unwrap();
    println!("shard_id_bytes: {:?}", shard_id_bytes);
    u256_to_bytes32(shard_id, shard_id_bytes);

    // Supply data to be hashed to the message digest object 
    // by creating calls to an update method
    sha3.update(shard_id_bytes);
    // sha3.update(&shard_id_bytes[..]);

    // Add the parent hash
    /*
    let parent_hash_bytes: &mut [u8; 32] = [0x50, 0xa1, 0xb3, 0xd5, 0x14, 0xd4, 0x99, 0x63, 
                            0x54, 0x14, 0x7a, 0xd2, 0x89, 0x61, 0x75, 0xb0, 
                            0x7d, 0x43, 0x7f, 0x9e, 0x58, 0xfa, 0x3c, 0x44, 
                            0x86, 0xc0, 0x42, 0xf4, 0xc3, 0xd5, 0x05, 0x9b];
    // let parent_hash_bytes: &mut [u8; 32] = &mut [0; 32];
    parent_hash.copy_to(parent_hash_bytes);
    sha3.update(parent_hash_bytes);
    // sha3.update(&parent_hash_bytes[..]);
    */

    // Add the chunk root
    
    let chunk_root_bytes: &mut [u8; 32] = &mut [0x50, 0xce, 0xc0, 0x49, 0x54, 0x77, 0xfb, 0x7e,
                                        0x65, 0x25, 0xc2, 0xa0, 0x39, 0xa3, 0xa9, 0x95, 
                                        0x34, 0x90, 0x35, 0xb2, 0xa8, 0x23, 0xa4, 0x99,
                                        0x0b, 0x27, 0xf6, 0xd7, 0xd5, 0x5e, 0xec, 0x6b];
    // let chunk_root_bytes: &mut [u8; 32] = &mut [0; 32];
    
    let chunk_root = ethereum_types::H256::from_slice(&chunk_root_bytes[..]);

    // Copy data of object into mutable slice of length len()
    chunk_root.copy_to(chunk_root_bytes);
    println!("chunk_root_bytes: {:?}", chunk_root_bytes);
    println!("chunk root: {:?}", chunk_root);
    println!("chunk_root_bytes with chunk_root: {:?}", chunk_root_bytes);
    sha3.update(chunk_root_bytes);
    // sha3.update(&chunk_root_bytes[..]);

    // Add Period
    let period_bytes: &mut [u8; 32] = &mut [0; 32];
    let period = ethereum_types::U256::from_dec_str("1").unwrap();
    u256_to_bytes32(period, period_bytes);
    sha3.update(period_bytes);
    // sha3.update(&period_bytes[..]);

    // Add Proposer Address
    let proposer_address_bytes: &mut [u8; 20] = &mut [0x39, 0xa4, 0x2d, 0x47, 0x4a,
                                            0x52, 0x96, 0xab, 0x98, 0x52, 
                                            0x3b, 0x1a, 0x3d, 0xef, 0x8f, 
                                            0x18, 0x67, 0xad, 0x32, 0xb0];
    // let proposer_address_bytes: &mut [u8; 20] = &mut [0; 20];
    let proposer_address = ethereum_types::H160::from_slice(&proposer_address_bytes[..]);
    proposer_address.copy_to(proposer_address_bytes);
    // sha3.update(proposer_address_bytes);
    sha3.update(&proposer_address_bytes[..]);

    // Create header
    let collation_header_instance = Header::new(
        shard_id, /*ph,*/ 
        chunk_root, 
        period, 
        proposer_address
    );

    // Calculate its generated hash
    let header_hash = collation_header_instance.hash();

    // Finalize hash and return as H256
    let mut result_bytes: [u8; 32] = [0; 32];
    sha3.finalize(&mut result_bytes);

    let result = ethereum_types::H256::from_slice(&result_bytes[..]);

    // Ensure manually calculated hash matches the 
    // generated hash
    assert_eq!(result, header_hash);
}
