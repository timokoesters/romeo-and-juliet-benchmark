# Romeo and Juliet benchmark

Usage:

1. Edit src/main.rs to point to the correct homeserver and room id.

2. Run these commands:

```
cargo build --release
time cargo run --release
```

This will go through the play defined in romeo_and_juliet.txt and create users
for each character and sends one /send request for each line they say.

## Results (2020-08-19):

Synapse:
```
# default
time 5m0.870s
```

Dendrite:
```
# default
time 6m8.802s

# postgres:
time 2m45.387s
```


Conduit:
```
# default
time 0m4.184s
```

## Contact me:

Matrix: @timo:koesters.xyz
