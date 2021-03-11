`digitalocean_playbook.yml` needs ansible-galaxy module `community.digitalocean`, general roles need `community.docker`, install with;

```
ansible-galaxy collection install community.docker community.digitalocean
```

Add a `DO_API_TOKEN` with a digitalocean `write` API token to your environment variables before running the playbook.

Parameters can be adjusted in `group_vars/servers.yml`

Note: Due to the way digitalocean charges you for droplets,
each droplet will be minimally charged for one cent,
meaning you'll have to pay `droplets * 0.01` cents for each run of the playbook,
with however many droplets you have created and destroyed during the run.