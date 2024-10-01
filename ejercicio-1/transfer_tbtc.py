from cryptos import Bitcoin
import json 

coin = Bitcoin(testnet=True)

f = open('wallet.json', 'r')
raw = f.read()
wallet_data = json.loads(raw)
f.close()

wallet = coin.wallet(wallet_data["seed"])

address = wallet.receiving_address(0)
privkey = wallet.privkey(address)

change = wallet.change_address(0)

print("Transfering from address:", address)
print("Transfering to address:", change)

tx = coin.send(privkey, address, change, 20)
print("TX:", tx)

