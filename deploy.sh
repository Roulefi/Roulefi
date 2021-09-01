publisher='bhc3.testnet'
user='bhc.testnet'
prefix='roulette'

yarn build:contract
yarn postbuild

contract_id="$prefix.$publisher"

near delete "$contract_id" "$publisher"
near create-account "$contract_id" --masterAccount "$publisher"

near deploy --accountId "$contract_id"
near call "$contract_id" new --accountId "$publisher"