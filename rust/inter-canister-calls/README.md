# Inter-canister calls

This project contains the code from the [inter-canister call Rust tutorial](https://internetcomputer.org/docs/building-apps/developer-tools/cdks/rust/intercanister). Please follow the tutorial for explanations.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/inter-canister-calls)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
