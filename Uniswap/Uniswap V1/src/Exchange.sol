    // SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract Exchange is ERC20 {
    address public tokenAddress;

    constructor(address tokenAddr) ERC20("ETH TOKEN LP Token", "lpETHTOKEN") {
        require(tokenAddr != address(0), "Invalid token address");
        tokenAddress = tokenAddr;
    }

    // getReserve returns the balance of `token` held by `this` contract
    function getReserve() public view returns(uint256) {
        return ERC20(tokenAddress).balanceOf(address(this));
    }
}
