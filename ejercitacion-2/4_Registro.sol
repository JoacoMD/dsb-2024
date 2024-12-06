// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

contract Registro {

    struct User {
        string name;
        bool exists;
    }

    mapping(address => User) users;

    function register(string memory name) public {
        User storage user = users[msg.sender];
        require(!user.exists, "User already registered");
        user.name = name;
        user.exists = true;
    }

    function getUser(address addr) public view returns (string memory) {
        User memory user = users[addr];
        require(user.exists, "User is not registered");
        return user.name;
    }

    function userExists(address addr) external view returns (bool) {
        return users[addr].exists;
    }
}

contract Muro {
    address private contractRegister;

    struct Message {
        address author;
        string content;
    }

    Message[] public muro;
    
    constructor(address registro) {
        contractRegister = registro;
    }

    function publicar(string memory msj) public {
        Registro reg = Registro(contractRegister);
        bool userExists = reg.userExists(msg.sender);
        require(userExists, "User must be registered");
        Message memory mesg = Message(msg.sender, msj);
        muro.push(mesg);
    }

}