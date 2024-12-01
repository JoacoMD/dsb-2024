from brownie import Votacion, accounts, config, network, reverts
import pytest 

@pytest.fixture
def votacion():
    if network.show_active() == 'development':
        account = accounts[0]
        return Votacion.deploy(['Trump', 'Harris'], {'from': account})
    elif network.show_active() == 'sepolia':
        return Votacion[-1]
    
def test_vote(votacion):
    account = accounts[0]
    tx = votacion.vote(0, {'from': account})
    tx.wait(1)
    assert tx.status == 1
    winner = votacion.winningProposal()
    assert winner == 0

def test_double_vote(votacion):
    account = accounts[0]
    tx = votacion.vote(0, {'from': account})
    tx.wait(1)
    assert tx.status == 1
    with reverts("Already voted."):
        votacion.vote(0, {'from': account})