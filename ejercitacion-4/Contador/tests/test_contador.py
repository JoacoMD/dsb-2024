from brownie import Contador, accounts, config, network
import pytest 

@pytest.fixture
def contador():
    if network.show_active() == 'development':
        account = accounts[0]
        return Contador.deploy({'from': account})
    elif network.show_active() == 'sepolia':
        return Contador[-1]
    
def test_retrieve_contador(contador):
    assert contador.retrieve() == 0

def test_whitelist(contador):
    account = accounts[0]
    tx = contador.addToWhitelist(account.address, {'from': account})
    tx.wait(1)
    assert tx.status == 1
    tx2 = contador.removeFromWhitelist(account.address, {'from': account})
    tx2.wait(1)
    assert tx2.status == 1

def test_increment(contador):
    account = accounts[0]
    tx = contador.addToWhitelist(account.address, {'from': account})
    tx.wait(1)
    tx = contador.increment({'from': account})
    tx.wait(1)
    assert tx.status == 1
    assert contador.retrieve() == 1

def test_increment_not_whitelisted(contador):
    account = accounts[0]
    with pytest.raises(Exception):
        contador.increment({'from': account})

def test_decrement(contador):
    account = accounts[0]
    tx = contador.addToWhitelist(account.address, {'from': account})
    tx.wait(1)
    tx = contador.increment({'from': account})
    tx.wait(1)
    tx = contador.decrement({'from': account})
    tx.wait(1)
    assert tx.status == 1
    assert contador.retrieve() == 0

def test_decrement_not_whitelisted(contador):
    account = accounts[0]
    with pytest.raises(Exception):
        contador.decrement({'from': account})