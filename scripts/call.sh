#!/bin/bash 

SIGNER=<signer-account>

source ./scripts/setting.conf

# Add student
near call $SUB_ACCOUNT add_to_map '{"name": "Jerry", "course": "CS", "year": 4, "age": 75}' --accountId $SIGNER --amount 0

# Get student
# near call $SUB_ACCOUNT get_from_map '{}' --accountId $SIGNER

# Delete student
# near call $SUB_ACCOUNT remove_from_map '{}' --accountId $SIGNER

# Search student key
# near view $SUB_ACCOUNT search_in_map '{"account_id": "paul-otieno.testnet"}'

# Add to Vector
# near call $SUB_ACCOUNT add_to_vec '{"name": "Software Engineering"}' --accountId $SIGNER

# Get from vector
# near view $SUB_ACCOUNT get_from_vec '{"start": 0, "limit": 5}'

# Remove from vector
# near call $SUB_ACCOUNT remove_from_vec '{"index": 0}' --accountId $SIGNER
