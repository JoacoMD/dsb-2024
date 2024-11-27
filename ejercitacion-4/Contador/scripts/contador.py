from brownie import Contador
from .utils import get_account, get_contract

def deploy():
    account = get_account()
    Contador.deploy({'from': account})

def verify():
    contador = get_contract()
    Contador.publish_source(contador)
