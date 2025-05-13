# Some initialization experiments

This workspace consists of two libraries:

- `initialization` defines the main types and has no dependencies (except `linkme`)
- `my_plugin` depends on `initialization`

The `Age` plugin is defined in `initialization`, and the `Weight` plugin is defined in `my_plugin`. 

Run the test in `initialization`:

```
cargo test --package initialization --lib test -- --show-output
# Output: Plugin: Age
```

Then run the test in `my_plugin`:

```
cargo test --package my_plugin --lib test -- --show-output
# Output: Plugin: Weight
#         Plugin: Age
```

This demonstrates that defining plugins across crate boundaries works. 
