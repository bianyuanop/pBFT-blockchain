use std::collections::HashMap;

use blake3::Hash;
use libp2p::{PeerId, identity::{self, error::SigningError}};
use super::transaction::Transaction;

pub struct BlockHeader {
    pub block_num: u128,
    pub checksum: Hash,
    pub signatures: HashMap<PeerId, super::Signature>,
    pub block_time: u128,
    pub prev_blockhash: Hash
}

pub struct Block {
    header: BlockHeader,
    txns: Vec<Transaction>
}

impl Block {
    fn hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();
        
        for tx in self.txns.iter() {
            hasher.update(tx.checksum.as_bytes());
        }

        hasher.finalize()
    }

    fn hash_and_set(&mut self) -> () {
        self.header.checksum = self.hash();         
    }

    // fn sign(&mut self, known_peers: Vec<PeerId>, keypair: identity::Keypair) -> Result<(), SigningError> {
    //     let known_peer_count: u32 = 0;

    //     for peer in known_peers.iter() {
    //         let pubkey = identity::PublicKey::from(value)
    //     } 


    //     Ok(())
    // }

    fn push_tx(&mut self, tx: &Transaction) -> () {
        self.txns.push(tx.clone());
    }
}