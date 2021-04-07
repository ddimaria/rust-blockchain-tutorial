// contracts/GLDToken.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract RustCoinToken is ERC20 {
    constructor(uint256 initialSupply) public ERC20("RustCoin", "RC") {
        _mint(msg.sender, initialSupply);
    }
}
