# Gradient Slice

gradient slice is a safe crate to iterate over a gradient of permutations of slices of a Vec.

## Examples

```
use gradient_slice::Gradient;
let boot = Gradient::new(0x1BADB002u32.to_be_bytes().to_vec())
    .map(Vec::from)
    .collect::<Vec<Vec<u8>>>();
assert_eq!(
    boot,
    vec![
        vec![27], vec![173], vec![176], vec![2],
        vec![27, 173], vec![173, 176], vec![176, 2],
        vec![27, 173, 176], vec![173, 176, 2],
        vec![27, 173, 176, 2]
    ]
);

let alphabet = Gradient::new(" abc ".chars().collect::<Vec<char>>())
    .map(Vec::from)
    .map(|vec| {
        vec.iter()
            .map(Clone::clone)
            .map(String::from)
            .collect::<String>()
    })
    .collect::<Vec<String>>();
assert_eq!(
    alphabet,
    vec![
        " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
        "abc ", " abc "
    ]
);
```
