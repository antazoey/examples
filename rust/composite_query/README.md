# Composite queries

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup the project environment

We first need to build the data partition backend canister.

```bash
cd examples/rust/composite_query
dfx start --background
dfx canister create data_partition --no-wallet
dfx build data_partition
```

During the compilation of the fronted canister, the canister's wasm code will be inlined in the frontend canister's wasm code.

```bash
dfx canister create kv_frontend --no-wallet
dfx build kv_frontend
dfx canister install kv_frontend
```

## Step 2: Using the canister

Now we add some key value pairs via the frontend canister.

```bash
dfx canister call kv_frontend put '(1:nat, 1337:nat)'
(null)
dfx canister call kv_frontend put '(1:nat, 42:nat)'
(opt (1_337 : nat))
```

Note that the first call to `put` is slow, since the data partitions have to be created first.

```bash
dfx canister call kv_frontend get '(1:nat)'
(opt (42 : nat))
```

We can also query it via a (duplicate) method by doing update calls:

```bash
dfx canister call kv_frontend get_update '(1:nat)'
(opt (42 : nat))
```

It's also possible to do *two* query calls, first into the frontend and then into the data partition:

```bash
$ dfx canister call kv_frontend lookup '(1: nat)'
(1 : nat, "dmalx-m4aaa-aaaaa-qaanq-cai")
$ dfx canister call dmalx-m4aaa-aaaaa-qaanq-cai get '(1: nat)' --query
(42 : nat)
```
