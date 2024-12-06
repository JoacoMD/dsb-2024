// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

/**
 * @title Wallet
 * @dev Simple wallet
 */
contract Wallet {

    address private owner;

    // modifier to check if caller is owner
    modifier isOwner() {
        // If the first argument of 'require' evaluates to 'false', execution terminates and all
        // changes to the state and to Ether balances are reverted.
        // This used to consume all gas in old EVM versions, but not anymore.
        // It is often a good idea to use 'require' to check if functions are called correctly.
        // As a second argument, you can also provide an explanation about what went wrong.
        require(msg.sender == owner, "Caller is not owner");
        _;
    }

    /**
     * @dev Set contract deployer as owner
     */
    constructor() {
        owner = msg.sender; 
    }

    function deposit() public payable {
        require(msg.value > 0, "Debe enviar algo de ETH");
    }

    receive() external payable { }

    function retire(address payable to, uint256 amount) public isOwner {
        uint balance = address(this).balance;
        require(amount <= balance, "Monto a retirar no debe ser mayor a balance");
        bool result = to.send(amount);
        require(result, "Fallo el retiro de ETH");
    }

    function balance() public view isOwner returns (uint256) {
        return address(this).balance;
    }
} 
