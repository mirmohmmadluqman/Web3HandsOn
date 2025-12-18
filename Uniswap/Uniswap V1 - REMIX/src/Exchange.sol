// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

// import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
// import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol";
import "https://raw.githubusercontent.com/OpenZeppelin/openzeppelin-contracts/v5.0.2/contracts/token/ERC20/ERC20.sol";

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

    function addLiquidity(uint256 amountOfToken) public payable returns(uint256) {
        uint256 lpTokensToMint;
        uint256 ethReserveBalance = address(this).balance;
        uint256 tokenReserveBalance = getReserve();
        
        ERC20 token = ERC20(tokenAddress); // Binding the ABI to that address

        if (tokenReserveBalance == 0) {
            token.transferFrom(msg.sender, address(this), amountOfToken); // Transfer the token from the user to the exchange

            // lpTokensToMint = ethReserveBalance = msg.value
            lpTokensToMint = ethReserveBalance;

            _mint(msg.sender, lpTokensToMint);

            return lpTokensToMint;
        }

        uint256 ethReservePriorToFunctionCall = ethReserveBalance - msg.value;
        uint256 mintTokenAmountRequired = (msg.value * tokenReserveBalance) /
            ethReservePriorToFunctionCall;
    }   
}
