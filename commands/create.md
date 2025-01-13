
### Create ContractInfo model

```sh
cargo loco generate model contract package_id:string! registry_id:string! network:string! is_active:bool! version:int!
```

### Create a Pool model

```sh
cargo loco generate model pool pool_id:string! start_time:tstz! end_time:tstz! drawn_time:tstz! contract:references
```

