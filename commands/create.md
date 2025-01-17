
### Create Contract model

```sh
cargo loco generate model contract package_id:string! registry_id:string! vault_id:string! create_cap_id:string! registry_initial_version:int! network:string! is_active:bool! version:int!
```

### Add vault_id to Contract model

```sh
cargo loco g migration AddVaultIdAndCreateCapIdAndRegistryInitialVersionToContracts vault_id:string! create_cap_id:string! registry_initial_version:int!

cargo loco db migrate
```

### Add draw_cap_id to Contract model

```sh
cargo loco g migration AddDrawCapIdToContracts draw_cap_id:string!

cargo loco db migrate

cargo loco db entities
```

### Create a Pool model

```sh
cargo loco generate model pool pool_id:string! price:int! type_name:string! pool_type:string! start_time:tstz! end_time:tstz! drawn_time:tstz lucky_number:string round:int epoch:int is_active:bool! contract:references
```

