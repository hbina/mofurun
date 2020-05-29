## 🧸 Mofurun 🧸

🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸

Experimental implementation of `Vec` that stores the state of the underlying array using types.

This allows us to optimize some operations based on its state.

🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸🧸

# Example

For the simplest case, consider finding the maximum value in a vector.

```rust
let v = SortedVec::new();
v.push(3); // Pushing an item possibly breaks the invariant that `v` is sorted. This function therefore returns `UnsortedVec`
v.push(2);
let m = v.max(); // This is O(n).
let a = v.sort(); // Calling sort on `v` will recover the invariant. This function returns a `SortedVec`.
let n = a.max(); // This is O(1) because its a sorted vector.
```