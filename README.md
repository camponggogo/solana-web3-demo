# Solana Web3 Example

there are some solana web3 example

## Guide

### Tour

*  1.[create keypair](tour/create-keypair/main.en.ts)
*  2.[retrieve keypair](tour/retrieve-keypair/main.en.ts)
*  3.[build connection](tour/create-connection/main.en.ts)
*  4.[request airdrop](tour/request-airdrop/main.en.ts)
*  5.[get sol balance](tour/get-sol-balance/main.en.ts)
*  6.[sol transfer](tour/transfer/main.en.ts)
*  7.[create mint](tour/create-mint/main.en.ts)
*  8.[get mint info](tour/get-mint/main.en.ts)
*  9.[create token naccount](tour/create-token-account/main.en.ts)
* 10.[mint token](tour/mint-to/main.en.ts)
* 11.[get token balance](tour/get-token-balance/main.en.ts)
* 12[token transfer](tour/token-transfer/main.en.ts)

### Advanced

* 1.[Token](advanced/token/README.en.md)
  * 1.1.[Close account](advanced/token/close-account/main.en.ts)
  * 1.2.[Get all token accounts by owner](advanced/token/get-all-token-account-by-owner/main.en.ts)
  * 1.3.[Wrapped SOL](advanced/token/wrapped-sol)
* 2.[NFT](advanced/metaplex)
* 3.[Durable Nonce](advanced/durable-nonce/README.en.md)
* 4.[Send Tx](advanced/send-tx/main.ts)
* 5.[interact with program](advanced/interact-with-program)

### Solana Cluster RPC Endpoints
Solana maintains dedicated api nodes to fulfill [JSON-RPC](https://docs.solana.com/developing/clients/jsonrpc-api) requests for each public cluster, and third parties may as well. Here are the public RPC endpoints currently available and recommended for each public cluster:

Devnet#
Endpoint#
https://api.devnet.solana.com - single Solana-hosted api node; rate-limited
Rate Limits#
Maximum number of requests per 10 seconds per IP: 100
Maximum number of requests per 10 seconds per IP for a single RPC: 40
Maximum concurrent connections per IP: 40
Maximum connection rate per 10 seconds per IP: 40
Maximum amount of data per 30 second: 100 MB
Testnet#
Endpoint#
https://api.testnet.solana.com - single Solana-hosted api node; rate-limited
Rate Limits#
Maximum number of requests per 10 seconds per IP: 100
Maximum number of requests per 10 seconds per IP for a single RPC: 40
Maximum concurrent connections per IP: 40
Maximum connection rate per 10 seconds per IP: 40
Maximum amount of data per 30 second: 100 MB
Mainnet Beta#
Endpoints*#
https://api.mainnet-beta.solana.com - Solana-hosted api node cluster, backed by a load balancer; rate-limited
https://solana-api.projectserum.com - Project Serum-hosted api node
Rate Limits#
Maximum number of requests per 10 seconds per IP: 100
Maximum number of requests per 10 seconds per IP for a single RPC: 40
Maximum concurrent connections per IP: 40
Maximum connection rate per 10 seconds per IP: 40
Maximum amount of data per 30 second: 100 MB
*The public RPC endpoints are not intended for production applications. Please use dedicated/private RPC servers when you launch your application, drop NFTs, etc. The public services are subject to abuse and rate limits may change without prior notice. Likewise, high-traffic websites may be blocked without prior notice.

Common HTTP Error Codes#
403 -- Your IP address or website has been blocked. It is time to run your own RPC server(s) or find a private service.
429 -- Your IP address is exceeding the rate limits. Slow down! Use the Retry-After HTTP response header to determine how long to wait before making another request.

## Start

install dependency

```bash
npm install
```

run example

```bash
npx ts-node -s <PATH_FULL_FILE_HERE>
```

