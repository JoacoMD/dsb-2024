from brownie import Votacion
from .utils import get_account, get_contract

def deploy():
    account = get_account()
    Votacion.deploy(['Trump', 'Harris'],{'from': account})

def verify():
    contador = get_contract()
    Votacion.publish_source(contador)
