# AMM

A simple constant product curve (x.y = k) AMM. Where Liquidity providers can provide liquidity to choosen two tokens and the users can swap the tokens in exchange of other token.

# User flow

- Liquidity provider creates a pool with chosen two tokens and in return gets the lp token.
- Other liquidity providers can add liquidity to the already created pool and get lp tokens.
- User who wants mint-x tokens in exchange for mint-y, they can swap it only if pool is created already.

# Function

### `initialize_pool` has the `InitializePool` as the accounts and the following as the arguments

- `swap_fee` :- amount of fee to be set for each swap.

### `add_liquidity` has `Liquidity` as the accounts and the following as the arguments

- `amount_x` :- Maximum amount of mint-x wanted to deposit,
- `amount_y` :- Maximum amount of mint-y wanted to deposit,
- `amount_lp` :- Amount of lp tokens wanted in return.

### `remove_liquidity` has `Liquidity` as the accounts and the following as the arguments

- `amount_x` :- Minimum amount of mint-x wanted to be withdrawn,
- `amount_y` :- Minimum amount of mint-y wanted to be withdrawn,
- `amount_lp` :- Amount of lp tokens wanted in burned.

### `swap` has `Swap` as the accounts and the following as the arguments

- `amount_x`:- amount of mint-x tokens,
- `amount_y`:- amount of mint-y tokens,
- `is_x`:- mint-x tokens if true else mint-y.

# Functions behaviour

### initialize_pool

### add_liquidity

### remove_liquidity

### swap
