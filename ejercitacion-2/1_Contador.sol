// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

/**
 * @title Contador
 * @dev Contador con whitelist
 */
contract Contador {

    uint256 number;
    address private owner;
    mapping(address => bool) private whitelist;

    constructor() {
        owner = msg.sender;
    }

    modifier isOwner() {
        require(msg.sender == owner, "Caller is not owner");
        _;
    }

    modifier isWhitelisted() {
        require(whitelist[msg.sender], "Address is not in whitelist");
        _;
    }

    event CounterChanged (address indexed by, uint256 oldValue, uint256 newValue);

    function addToWhitelist(address newAddress) public isOwner {
        require(!whitelist[newAddress], "Address already in whitelist");
        whitelist[newAddress] = true;
    }

    function removeFromWhitelist(address removeAddress) public isOwner {
        require(whitelist[removeAddress], "Address is not in whitelist");
        whitelist[removeAddress] = false;
    }

    /**
     * @dev Incrementa valor de contador
     */
    function increment() public isWhitelisted {
        emit CounterChanged(msg.sender, number, number + 1);
        number = number + 1;
    }

    /**
     * @dev Decrementa valor de contador
     */
    function decrement() public isWhitelisted {
        emit CounterChanged(msg.sender, number, number - 1);
        number = number - 1;
    }

    /**
     * @dev Rentorna valor actual 
     * @return value of 'number'
     */
    function retrieve() public view returns (uint256){
        return number;
    }
}