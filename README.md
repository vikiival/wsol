## Wrapped Solana

Minimal contract for wrapping Solana tokens into Wrapped Solana (wSOL) tokens.
This ensures that you can use WSOL on decentralized exchanges and other DeFi
applications.

## [You can use it directly on Solana Playground](https://beta.solpg.io/https://github.com/vikiival/wsol)

> [!IMPORTANT]
> Read Instructions how to use it on Solana Playground / how to useinputs

## Inputs

| Who                | Value                                                                                     |
| ------------------ | ----------------------------------------------------------------------------------------- |
| `payer`            | `<your_address>`                                                                          |
| `wrap_sol_account` | `seed: [<your_address>, <token_program>, <wrapped_sol_addr>] + program<associated_token>` |
| `wsol_mint`        | `wrapped_sol_addr`                                                                        |
| `token_program`    | `token_program`                                                                           |
| `system_program`   | `system_program`                                                                          |


### Ref(s)

> [!NOTE]
> Kudos to @RostarMarek, he helped me with debugging and inputs 


