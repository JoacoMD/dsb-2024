from web3 import Web3
import os

with open("MuroABI.json") as f:
    abi = f.read()

contract_address = "0x8054c33BDAe0f1Bf77D9cb1bEfD86632C66C7067"

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