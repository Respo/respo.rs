## RespoState derive macro

to be used like:

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, RespoState)]
struct PanelState {
  content: String,
}
```

find more details in https://crates.io/crates/respo .
