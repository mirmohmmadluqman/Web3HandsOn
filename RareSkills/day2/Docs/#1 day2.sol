// Today we will learn how to create a Solana program that accomplishes 
// the same things as the Solidity contract below. We will also learn how
// Solana handles arithmetic issues like overflow.

contract Day2 {

	event Result(uint256);
	event Who(string, address);
	
	function doSomeMath(uint256 a, uint256 b) public {
		uint256 result = a + b;
		emit Result(result);
	}

	function sayHelloToMe() public {
		emit Who("Hello World", msg.sender);
	}
}
