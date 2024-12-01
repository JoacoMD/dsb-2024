from brownie import Votacion, accounts, network, config

def get_account():
    if network.show_active() == 'development':
        return accounts[0]
    elif network.show_active() == 'sepolia':
        return accounts.add(config['deployer_sepolia'])
    
def get_contract():
    if network.show_active() == 'development':
        account = accounts[0]
        return Votacion.deploy({'from': account})
    elif network.show_active() == 'sepolia':
        return Votacion[-1]