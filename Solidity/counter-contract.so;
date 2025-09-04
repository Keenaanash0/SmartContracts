// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint256 private counter;
    string private description;

    constructor(string memory _description) {
        counter = 0;
        description = _description;
    }

    function increment() public {
        counter += 1;
    }

    function getCounter() public view returns (uint256) {
        return counter;
    }

    function getDescription() public view returns (string memory) {
        return description;
    }
}
