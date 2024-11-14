from RegistroClient import contract, w3
from web3 import Web3
import os 

account = w3.eth.account.from_key(os.environ.get("pk"))
print(contract.functions.getUser(Web3.to_checksum_address(account.address)).call())