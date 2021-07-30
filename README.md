# Romeo and Juliet benchmark

## Usage:

Run this command with the correct port:

```
cargo run --release -- http://localhost:8000
```

This will go through the play defined in romeo_and_juliet.txt and create users
for each character and sends one /send request for each line they say.


## Results (2021-07):

| Tester         | OS    | CPU              | Storage  | Synapse sqlite  | Dendrite sqlite     | Dendrite postgres | Conduit sqlite      | Conduit heed (LMDB) |
| -------------- | ----- | ---------------- | -------- | --------------- | ------------------- | --------------    | ------------------- | ------------------- |
| timo           | linux | Intel i7 2nd gen | SATA SSD | -               | 8m24s               | 3m54s             | 2m13s               | 1m24s               |
| neilalexander  | macOS | Intel i7         | NVMe SSD | -               | 0m57s               | -                 | -                   | -                   |
| aaron          | macOS | Intel i7 7nd gen | NVMe SSD | -               | 0m51s               | -                 | 0m28s               | -                   |


### Reproducible runs (contributed by ShadowJonathan)

While [contrib](contrib/README.md) contains some rudimentary documentation, to be able to use it, you must first:
 - Have `ansible-playbook` installed
 - Have a DigitalOcean account (with billing info registered, or using a budget)
 - Have one of your public keys added to digitalocean (it's also important that your local machine tries to log into servers with this automatically)
 - Install the `ansible-galaxy` collections documented in [contrib's readme](contrib/README.md)
 - Add the DigitalOcean API token to your environment (also as documented in [contrib's readme](contrib/README.md))

Then, `cd` into `contrib/`, and run
```
ansible-playbook digitalocean_playbook.yml
```

This'll run the tests in parallel on digitalocean droplets, results will be put into `contrib/out/`,
you can adjust which server versions or revisions are ran/built using `contrib/group_vars/server.yml`,
the committed file is "known to work", and can always be rolled back to when something goes wrong.

Some stages of the playbook could take a while, such as building `rjbench`, or `conduit`, or the testing itself.
It's advised to leave it alone when it does this, although, if a stage takes more than an hour (as a rule of thumb),
or if you're curious what's happening, take one of the IP addresses noted during the "Create droplets" stage, do `ssh root@IP_ADDRESS`,
and look around with `htop` and `journalctl -f -n 100`.


## Contact me:

Matrix: @timo:koesters.xyz
