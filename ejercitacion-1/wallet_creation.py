from cryptos import entropy_to_words, Bitcoin
import os
import json 

words = entropy_to_words(os.urandom(16))
print(words)

coin = Bitcoin(testnet=True)
wallet = coin.wallet(words)

addr1 = wallet.new_receiving_address()
privkey1 = wallet.privkey(addr1)

print("publica:", addr1)
print("privada:", privkey1)

addr2 = wallet.new_change_address()
privkey2 = wallet.privkey(addr2)

print("publica:", addr2)
print("privada:", privkey2)

w = {
    "seed": words,
    "addr1": {
        "privkey": privkey1,
        "addr": addr1,
    },
    "addr2": {
        "privkey": privkey2,
        "addr": addr2,
    }
}

with open("wallet.json", "w") as outfile:
    json.dump(w, outfile)