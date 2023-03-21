use blake3;
use libp2p::{identity::{self, error::SigningError}};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub opcode: u128,
    pub payload: Vec<u8>,
    pub checksum: blake3::Hash,
}

impl Transaction {
    fn hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();

        hasher.update(&self.opcode.to_be_bytes());
        hasher.update(&self.payload[..]);

        hasher.finalize()
    }

    fn hash_and_set(&mut self) -> () {
        self.checksum = self.hash();
    }
}

#[cfg(test)]
mod tests {
    use super::Transaction;

    #[test] 
    fn hash_and_verify() {
        let mut tx = Transaction {
            opcode: 0,
            payload: vec![],
            checksum: blake3::hash(&[])
        };

        tx.hash_and_set();

        assert_eq!(tx.checksum, tx.hash());
    }
}