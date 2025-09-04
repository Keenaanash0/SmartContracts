// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ProposalContract {
    struct Proposal {
        string title;
        string description;
        uint256 yesVotes;
        uint256 noVotes;
    }

    Proposal[] public proposals;

    function createProposal(string memory _title, string memory _description) public {
        proposals.push(Proposal({
            title: _title,
            description: _description,
            yesVotes: 0,
            noVotes: 0
        }));
    }

    function vote(uint256 index, bool support) public {
        require(index < proposals.length, "Proposal does not exist");
        if (support) {
            proposals[index].yesVotes += 1;
        } else {
            proposals[index].noVotes += 1;
        }
    }

    function getProposal(uint256 index) public view returns (string memory, string memory, uint256, uint256) {
        require(index < proposals.length, "Proposal does not exist");
        Proposal storage proposal = proposals[index];
        return (proposal.title, proposal.description, proposal.yesVotes, proposal.noVotes);
    }

    // Custom logic for proposal state
    function getProposalState(uint256 index) public view returns (string memory) {
        require(index < proposals.length, "Proposal does not exist");
        Proposal storage proposal = proposals[index];

        if (proposal.yesVotes >= proposal.noVotes + 2) {
            return "Succeeded";
        } else if (proposal.noVotes > proposal.yesVotes) {
            return "Failed";
        } else {
            return "Pending";
        }
    }
}
