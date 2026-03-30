# CropPay
Instant cooperative tokenization for rice farmers in the Philippines.

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
