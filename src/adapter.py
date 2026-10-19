"""ZenGo Threshold Cryptography (MPC) Wallet Engine."""
def generate_threshold_shares(secret: int, threshold: int = 2) -> list:
    shares = []
    for i in range(1, threshold + 2):
        shares.append((i, (secret + i * 17) % 65537))
    return shares
