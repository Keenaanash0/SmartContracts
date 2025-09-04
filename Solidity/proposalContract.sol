// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ProposalContract {
    struct Proposal {
        string title;        
        string description;  
    }

    Proposal[] public proposals;

    function createProposal(string memory _title, string memory _description) public {
        proposals.push(Proposal({
            title: _title,
            description: _description
        }));
    }

    function getProposal(uint256 index) public view returns (string memory, string memory) {
        require(index < proposals.length, "Proposal does not exist");
        Proposal storage proposal = proposals[index];
        return (proposal.title, proposal.description);
    }
}
