# K1 Yield Vault (Anchor)

New Anchor program implementing a USDC-denominated, share-based multi-strategy vault.

## Implemented

- Vault + strategy PDAs
- Deposit / withdraw with share accounting
- Mint/withdraw/performance fee mechanics (share-based)
- Strategy add/deactivate/allocate/deallocate/report hooks
- Off-chain NAV updates
- Pause / unpause / emergency strategy withdraw
- u128 checked math helpers and unit tests

## Notes

- This is a clean-slate program scaffold independent from `reflect-tokenised-bonds`.
- Strategy CPI interfaces are modeled as account + transfer/report hooks and can be expanded with concrete strategy CPIs.
