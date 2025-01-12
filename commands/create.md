
### Create a Pool model

```sh
cargo loco generate model pool pool_id:string start_time:ts end_time:ts drawn_time:ts created_time:ts created_by:string
```

### Create ContractInfo model

```sh
cargo loco generate model contract_info package_id:string! registry_id:string! network:string! is_active:bool!