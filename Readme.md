# TagTeam statistics

## Disclaimer
I am not affiliated with the board/card game TagTeam. 

This is not a simulation to play this card game online.

The intention of this project is to provide a service for fellow gamers to collect data on the great heroes of this game to analyze later.

## Dev setup
This project uses the [Rust on Nails](https://rust-on-nails.com/) architecture as well as their provided devcontainer.

Please follow their [documentation](https://rust-on-nails.com/docs/ide-setup/dev-env-as-code/) to use the dev container.

The Justfile has been adjusted to install additional dev dependencies like openapi-generator.  
By following the Rust on Nails tutorial you will automatically prepare these changes as well.  
It will also generate the current openapi and db client.

It will install the npm version of the [openapi-generator-cli](https://openapi-generator.tech/docs/installation).

## What you should NOT do
* Manually change files in the [openapi crate](crates/openapi) (e.g. db clients)  
Instead:  
  1. Change the [api.yaml](openapi/api.yaml)
  2. Run `just generate-openapi` from root

* Manually change files in the [clorinde crate](crates/clorinde)  
Instead:
  1. Change the [queries](crates/db/queries/)
  2. Run `just generate-db-client` from root

 

