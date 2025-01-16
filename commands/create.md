
### Create Contract model

```sh
cargo loco generate model contract package_id:string! registry_id:string! network:string! is_active:bool! version:int!
```

### Add vault_id to Contract model

```sh
cargo loco g migration AddVaultIdToContracts vault_id:string!
```


### Create a Pool model

```sh
cargo loco generate model pool pool_id:string! price:int! type_name:string! pool_type:string! start_time:tstz! end_time:tstz! drawn_time:tstz lucky_number:string round:int epoch:int is_active:bool! contract:references
```

