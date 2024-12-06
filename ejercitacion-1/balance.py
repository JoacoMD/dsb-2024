from cryptos import Bitcoin
import json 

coin = Bitcoin(testnet=True)

f = open('wallet.json', 'r')
raw = f.read()
wallet_data = json.loads(raw)
f.close()

wallet = coin.wallet(wallet_data["seed"])

address = wallet.receiving_address(0)
change = wallet.change_address(0)

balance = coin.get_balance(address)

print('Balance:', balance['confirmed'])
print('Unconfirmed Balance:', balance['unconfirmed'])

balance_change = coin.get_balance(change)

print('Balance Change:', balance_change['confirmed'])
print('Unconfirmed Balance Change:', balance_change['unconfirmed'])
