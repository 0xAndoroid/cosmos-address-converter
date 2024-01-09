# Cosmos Address Converter
With the uprising of cosmos ecosystem airdrops I was wondering whether it is possible to
convert from address on one IBC chain to an address on another IBC chain.  
*project that want to airdrop could use this to protect against sybil attacks*
  
With some research, turns out, it is possible, that's exactly the purpose of this binary.

## Usage
```bash
cargo run -- <address that you know> <prefix of other chain>
```
For example:
```bash
cargo run -- cosmos153tu72t7zynxt6ahha5cp2pjnxu807mdxwr9ua celestia
# output is celestia153tu72t7zynxt6ahha5cp2pjnxu807mdhyj4xs
```

## Limitations
Well, not all IBC chains follow the same address structure. The one notable example is EvmOS.  
Instead of encoding *processed* public key into Bech32 format, they encode Ethereum address
*(which is also a processed public key, but in a different way, you got the idea)*.  
**This crate supports convertations from ethereum addresses to evmos addresses**
```bash
cargo run -- <0x ethereum address> evmos
```

## Explanation for those who are interested
Most IBC addresses are in Bech32 format. In this format we firstly get the public key,
then we hash it a couple of times to get what we will call `data`. Now, we encode this
data into Bech32 format. This format has a `hrp` - a prefix (like `cosmos`, or `celestia`).
A divisor (usually just `1`). The data itself, and the checksum of hrp and data. And the address is
`hrp || divisor || data || checksum`, where `||` is a concatination.  
Basically, this crate extracts data from inputted address, and encodes it with a different prefix.
