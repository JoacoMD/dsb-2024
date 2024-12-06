from RegistroClient import contract, w3
import os

account = w3.eth.account.from_key(os.environ.get("pk"))
nombre = input("Ingrese su nombre: ")

tx = {
"nonce": w3.eth.get_transaction_count (account.address),
"gasPrice": w3.eth.gas_price,
"value": 0,
"chainId": 11155111,
}

contract_data = contract.functions.register(nombre).build_transaction(tx)
estimated_gas = w3.eth.estimate_gas( contract_data )
tx['gas'] = estimated_gas
contract_data = contract.functions.register(nombre).build_transaction(tx)
signed_txn = w3.eth.account.sign_transaction( contract_data , os.environ.get("pk"))
txn_hash = w3.eth.send_raw_transaction( signed_txn.raw_transaction)

print("Registro exitoso")
print("Hash de la transacción:", txn_hash.hex())
