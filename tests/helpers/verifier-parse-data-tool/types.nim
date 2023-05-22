
import ../../../contracts/cosmos/verifier/lib/nim/contract_interactions/helpers


type
  InitDataEOS* = object
    key*: string
    verification_key*: string
    current_header_hash*: string
    current_slot*: uint64
    domain*: string
