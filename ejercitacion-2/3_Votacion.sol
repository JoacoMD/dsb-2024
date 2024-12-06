// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

/** 
 * @title Votacion
 */
contract Votacion {

    struct Voter {
        bool voted;
        uint vote; 
    }

    struct Candidate {
        string name; 
        uint voteCount; 
    }

    mapping(address => Voter) public voters;

    Candidate[] public candidates;

    /** 
     * @dev Crea una votacion con las opciones en 'candidatesNames'.
     * @param candidatesNames nombre de los candidatos
     */
    constructor(string[] memory candidatesNames) {

        for (uint i = 0; i < candidatesNames.length; i++) {
            candidates.push(Candidate({
                name: candidatesNames[i],
                voteCount: 0
            }));
        }
    }

    /**
     * @dev Votacion de un candidato pasando el indice
     * @param candidate indice del candidato en el array de candidatos
     */
    function vote(uint candidate) external {
        Voter storage sender = voters[msg.sender];
        require(!sender.voted, "Already voted.");
        sender.voted = true;
        sender.vote = candidate;

        candidates[candidate].voteCount += 1;
    }

    /** 
     * @dev Computes the winning proposal taking all previous votes into account.
     * @return winningProposal_ index of winning proposal in the proposals array
     */
    function winningProposal() public view
            returns (uint winningProposal_)
    {
        uint winningVoteCount = 0;
        for (uint p = 0; p < candidates.length; p++) {
            if (candidates[p].voteCount > winningVoteCount) {
                winningVoteCount = candidates[p].voteCount;
                winningProposal_ = p;
            }
        }
    }

    /** 
     * @dev Calls winningProposal() function to get the index of the winner contained in the proposals array and then
     * @return winnerName_ the name of the winner
     */
    function winnerName() external view
            returns (string memory winnerName_)
    {
        winnerName_ = candidates[winningProposal()].name;
    }
}