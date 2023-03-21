use libp2p::{identity};

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Signature, PublicKey};
    use libp2p::identity;

    #[test] 
    fn sign_and_verify() {
        let p2p_key = identity::Keypair::generate_ed25519();
        let pubkey = p2p_key.public();

        let msg: &[u8] = b"Hello, world";

        let sig = p2p_key.sign(msg).unwrap();

        assert_eq!(pubkey.verify(msg, &sig[..]), true);
    }
}