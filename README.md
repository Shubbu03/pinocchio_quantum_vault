# Pinocchio Quantum Vault

A Solana program implementing a quantum-resistant vault system using Winternitz signatures for post-quantum cryptography. Built with the Pinocchio framework for efficient, no-std Solana program development.

## Overview

The Quantum Vault provides a secure way to store and manage funds using quantum-resistant cryptographic signatures. Unlike traditional ECDSA signatures that are vulnerable to quantum attacks, this vault uses Winternitz signatures that remain secure even against quantum computers.

## Features

- **Quantum-Resistant Security**: Uses Winternitz signatures for post-quantum cryptography
- **Program Derived Addresses (PDA)**: Secure vault creation with deterministic addresses
- **Flexible Fund Management**: Split vaults with partial withdrawals or complete closure
- **No-Allocator Design**: Memory-efficient implementation using Pinocchio framework
- **Gas-Optimized**: Minimal compute units for cost-effective operations

## Instructions

### 1. Open Vault

Creates a new quantum vault with a unique PDA derived from the owner's public key.

**Accounts:**
- `payer` (signer, writable) - Account paying for vault creation
- `vault` (writable) - The vault account to be created
- `system_program` - Solana system program

**Instruction Data:**
```rust
struct OpenVault {
    hash: u8,    // Hash seed for PDA generation
    bump: u8,    // PDA bump seed
}
```

**Usage:**
```rust
// Create vault with specific hash and bump
let instruction_data = OpenVault {
    hash: 42,
    bump: 255,
};
```

### 2. Split Vault

Splits a vault by transferring a specified amount to a split account and refunding the remainder to a refund account. The vault is closed after the split.

**Accounts:**
- `vault` (writable) - The vault to be split
- `split` (writable) - Account receiving the split amount
- `refund` (writable) - Account receiving the remaining balance

**Instruction Data:**
```rust
struct SplitVault {
    signature: WinternitzSignature,  // Quantum-resistant signature
    amount: u8,                      // Amount to split (0-255 lamports)
    bump: u8,                        // PDA bump seed
}
```

**Security Features:**
- Signature verification using the split amount and account addresses
- PDA validation to ensure vault authenticity
- Atomic operation - either succeeds completely or fails

### 3. Close Vault

Completely closes a vault by transferring all remaining funds to a refund account.

**Accounts:**
- `vault` (writable) - The vault to be closed
- `refund` (writable) - Account receiving all remaining funds

**Instruction Data:**
```rust
struct CloseVault {
    signature: WinternitzSignature,  // Quantum-resistant signature
    bump: u8,                        // PDA bump seed
}
```

**Security Features:**
- Signature verification using the refund account address
- PDA validation to ensure vault authenticity
- Complete fund recovery

## Security Model

### Winternitz Signatures

The vault uses Winternitz signatures, which are:
- **Quantum-Resistant**: Secure against attacks by quantum computers
- **One-Time Use**: Each signature can only be used once
- **Deterministic**: Same input always produces the same signature

### PDA Validation

All operations validate the Program Derived Address using:
```rust
solana_nostd_sha256::hashv(&[
    recovered_pubkey_hash,
    bump_seed,
    program_id,
    "ProgramDerivedAddress"
])
```

### Message Construction

For split operations, the signature message includes:
- Amount to split (8 bytes)
- Split account public key (32 bytes)  
- Refund account public key (32 bytes)

## Dependencies

- `pinocchio` - Solana program framework
- `solana-winternitz` - Winternitz signature implementation
- `solana-nostd-sha256` - SHA256 hashing for PDA validation
- `shank` - IDL generation

## Building

```bash
chio build
```

## Testing

WIP

