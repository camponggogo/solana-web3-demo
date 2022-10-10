### Install the Solana Tool Suite
There are multiple ways to install the Solana tools on your computer depending on your preferred workflow:#
[https://docs.solana.com/cli/install-solana-cli-tools](https://docs.solana.com/cli/install-solana-cli-tools)

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
   ```bash
      $ solana config set --url https://api.testnet.solana.com
   ```
Rate Limits#
Maximum number of requests per 10 seconds per IP: 100
Maximum number of requests per 10 seconds per IP for a single RPC: 40
Maximum concurrent connections per IP: 40
Maximum connection rate per 10 seconds per IP: 40
Maximum amount of data per 30 second: 100 MB

Mainnet Beta#
Endpoints*#
https://api.mainnet-beta.solana.com - Solana-hosted api node cluster, backed by a load balancer; rate-limited
   ```bash
      $ solana config set --url https://api.mainnet-beta.solana.com
   ```
https://solana-api.projectserum.com - Project Serum-hosted api node
   ```bash
      $ solana config set --url https://solana-api.projectserum.com
   ```

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

# Solana Web3 Example

there are some solana web3 example

## Guide

### Tour

*  1.[create keypair](/tree/main/tour/create-keypair/main.en.ts)
 * ```bash
      npx ts-node -s /tour/create-keypair/private-key.ts
   ```
*  2.[retrieve keypair](/tree/main/tour/retrieve-keypair/main.en.ts)
 * ```bash
      npx ts-node -s /tour/retrieve-keypair/main.en.ts
   ```
*  3.[build connection](/tree/main/tour/create-connection/main.en.ts)
 * ```bash
      npx ts-node -s tour/create-connection/main.en.ts
   ```
*  4.[request airdrop](/tree/main/tour/request-airdrop/main.en.ts)
 * ```bash
      npx ts-node -s tour/request-airdrop/main.en.ts
   ```
*  5.[get sol balance](/tree/main/tour/get-sol-balance/main.en.ts)
 * ```bash
      npx ts-node -s tour/get-sol-balance/main.en.ts
   ```
*  6.[sol transfer](/tree/main/tour/transfer/main.en.ts)
 * ```bash
      npx ts-node -s tour/transfer/main.en.ts
   ```
*  7.[create mint](/tree/main/tour/create-mint/main.en.ts)
 * ```bash
      npx ts-node -s tour/create-mint/main.en.ts
   ```
*  8.[get mint info](/tree/main/tour/get-mint/main.en.ts)
 * ```bash
      npx ts-node -s tour/get-mint/main.en.ts
   ```
*  9.[create token naccount](/tree/main/tour/create-token-account/main.en.ts)
 * ```bash
      npx ts-node -s tour/create-token-account/main.en.ts
   ```
* 10.[mint token](/tree/main/tour/mint-to/main.en.ts)
 * ```bash
      npx ts-node -s tour/mint-to/main.en.ts
   ```
* 11.[get token balance](/tree/main/tour/get-token-balance/main.en.ts)
 * ```bash
      npx ts-node -s tour/get-token-balance/main.en.ts
   ```
* 12.[token transfer](/tree/main/tour/token-transfer/main.en.ts)
 * ```bash
      npx ts-node -s tour/token-transfer/main.en.ts
   ```

### Advanced

* 1.[Token](/tree/main/advanced/token/README.en.md)
  * 1.1.[Close account](/tree/main/advanced/token/close-account/main.en.ts)
   * ```bash
      npx ts-node -s advanced/token/close-account/main.en.ts
     ```
  * 1.2.[Get all token accounts by owner](/tree/main/advanced/token/get-all-token-account-by-owner/main.en.ts)
   * ```bash
      npx ts-node -s advanced/token/get-all-token-account-by-owner/main.en.ts
     ```
  * 1.3.[Wrapped SOL](/tree/main/advanced/token/wrapped-sol)
   * ```bash
      npx ts-node -s advanced/token/wrapped-sol/create-token-account/main.en.ts
      npx ts-node -s advanced/token/wrapped-sol/add-balance/main.en.ts
     ```
* 2.[NFT](/tree/main/advanced/metaplex)
  * ```bash
      npx ts-node -s advanced/metaplex/mint-nft/main.ts
      npx ts-node -s advanced/metaplex/mint-nft-new/main.ts
      npx ts-node -s advanced/metaplex/get-tokenmeta/main.ts
      npx ts-node -s advanced/metaplex/get-wallet-has-a-specific-nft/main.ts
    ```
* 3.[Durable Nonce](/tree/main/advanced/durable-nonce/README.en.md)
* 4.[Send Tx](/tree/main/advanced/send-tx/main.ts)
 * ```bash
      npx ts-node -s advanced/send-tx/main.ts
   ```
* 5.[interact with program](/tree/main/advanced/interact-with-program)

