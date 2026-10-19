use sha2::{Sha256, Digest};

pub struct ThresholdKeyShare {
    pub party_id: u16,
    pub secret_share: [u8; 32],
}

impl ThresholdKeyShare {
    pub fn sign_challenge(&self, message: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.secret_share);
        hasher.update(message);
        let mut sig = [0u8; 32];
        sig.copy_from_slice(&hasher.finalize());
        sig
    }
}
