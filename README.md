# Best-effort response demos

Simple examples to demonstrate how to issue and handle calls with best-effort responses work:

1. The `tryme` method can be called with any kind of call if the parameter is `> 0`, but only with a best effort response cal if the parameter is `0`. In particular, it will complain if it's called with a parameter of `0` from an ingress message.
2. The `demonstrate_timeouts` calls the `busy` method of the same canister with a short timeout (1 second). The `busy` method spins for several rounds, causing the timeout to elapse, and the call to fail with the new `SYS_UNKNOWN` reject code. The `demonstrate_timeouts` method returns `true` whenever this new code was returned.

The examples currently rely on `dfx` version `0.24.1-beta.1`, and the `next` branch of the Rust CDK. See `dfx.json` and `Cargo.toml` for how to set these versions.

To try:
```
$ dfx start
$ dfx deploy
```

Now you can try the generated Candid UI, or from the command line:

```
$ dfx canister call ber_backend tryme 1
```
The above command should succeed.

```
$ dfx canister call ber_backend tryme 0
```

The above command should fail.

```
$ dfx canister call ber_backend demonstrate_timeouts
```

The above command should return true if best-effort responses are working correctly.

