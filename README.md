# CropPay
Instant cooperative tokenization for rice farmers in the Philippines.

## Smart Contract Address
CDP54N72GGWVBR6ETK5L3GITKKPA6CR57UV3CHU43DT6UWCLBUS23IKG

## Stellar Expert Link
https://stellar.expert/explorer/testnet/contract/CDP54N72GGWVBR6ETK5L3GITKKPA6CR57UV3CHU43DT6UWCLBUS23IKG
<img width="1912" height="908" alt="A Jose_Deployed Contract" src="https://github.com/user-attachments/assets/cfc08667-ab2e-435a-a254-3d99bf92916a" />

## Problem
Rice farmers in Nueva Ecija wait 3–5 days for cash disbursement after delivering crops, missing input discounts.

## Solution
CropPay issues cooperative tokens instantly on Stellar via Soroban, redeemable for USDC at local anchors.

## Timeline
- Hackathon demo: 2 minutes from delivery entry to USDC wallet balance.

## Stellar Features Used
- USDC transfers
- Custom tokens
- Soroban smart contracts
- Trustlines

## Prerequisites
- Rust
- Soroban CLI v20+

## Build
```bash
soroban contract build

## Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/croppay.git
   cd croppay
   ```

2. **Configure Soroban CLI for Testnet**
   ```bash
   soroban config network add testnet \
     --rpc-url https://soroban-testnet.stellar.org \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

3. **Create or import a funded Testnet identity**
   ```bash
   soroban config identity generate alice
   soroban config identity fund alice --network testnet
   ```

## Usage

### Record a Delivery
```bash
soroban contract invoke \
  --id CDP54N72GGWVBR6ETK5L3GITKKPA6CR57UV3CHU43DT6UWCLBUS23IKG \
  --network testnet \
  --source alice \
  -- record_delivery \
  --farmer <FARMER_STELLAR_ADDRESS> \
  --amount 100
```

### Check Balance
```bash
soroban contract invoke \
  --id CDP54N72GGWVBR6ETK5L3GITKKPA6CR57UV3CHU43DT6UWCLBUS23IKG \
  --network testnet \
  --source alice \
  -- balance \
  --farmer <FARMER_STELLAR_ADDRESS>
```

### Redeem Tokens
```bash
soroban contract invoke \
  --id CDP54N72GGWVBR6ETK5L3GITKKPA6CR57UV3CHU43DT6UWCLBUS23IKG \
  --network testnet \
  --source alice \
  -- redeem \
  --farmer <FARMER_STELLAR_ADDRESS> \
  --amount 50
```

## How It Works

The Soroban contract exposes three functions:

**`record_delivery(farmer, amount)`** — Called when a farmer delivers rice. Mints cooperative tokens by incrementing the farmer's on-chain balance.

**`redeem(farmer, amount)`** — Called by the farmer to burn tokens. Deducts the balance and emits a `redeem` event that triggers a USDC payout.

**`balance(farmer)`** — Read-only query that returns the farmer's current cooperative token balance.

```
Farmer records delivery
        ↓
Soroban contract mints cooperative token
        ↓
Farmer swaps token for USDC via Stellar DEX
        ↓
Farmer redeems USDC at local anchor (cash out)
```
