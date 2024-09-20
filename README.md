# Best-effort response demos

Simple examples to check that best-effort responses work. Calling `tryme` on the backend with a parameter `i > 1`.

To try:
```
$ dfx start
$ dfx deploy
$ dfx canister call ber_backend tryme 1
$ dfx canister call ber_backend demonstrate_timeouts
```

The last one should return true once we have BER enabled

