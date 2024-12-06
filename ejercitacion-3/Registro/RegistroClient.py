from web3 import Web3
import os

with open("RegistroABI.json") as f:
    abi = f.read()

contract_address = "0x014D8f81BD800450A6dFCC659B34FBaC2f583297"

# NODE = os.environ.get("SEPOLIA_NODE")
NODE = "https://eth-sepolia.public.blastapi.io"
w3 = Web3(Web3.HTTPProvider(NODE))
if w3.is_connected():
    print("Connected to the node")
else:
    print("Error connecting to the node")
    quit()

account = w3.eth.account.from_key(os.environ.get("pk"))
contract = w3.eth.contract(address=contract_address, abi=abi)