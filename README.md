# Romeo and Juliet benchmark

## Usage:

Run these commands:

```
cargo build --release
time target/release/rjbench http://localhost:8000 $roomid:server.name
```

This will go through the play defined in romeo_and_juliet.txt and create users
for each character and sends one /send request for each line they say.

## Results (2020-08-19):

Synapse:
```
# default
time 5m0.870s

# postgres:
time 1m46.319s
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
