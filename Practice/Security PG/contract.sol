// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TestCall {
    string public message;
    uint public x;

    event Log(string message);

    // Fallback function must be declared as external
    fallback() external payable {
        emit Log("fallback was called");
    }

    function foo(string memory _message, uint _x) external payable returns (bool, uint) {
        message = _message;
        x = _x;
        return (true, 999);
    }
}

contract Call {
    bytes public data;

    // Function to call 'foo' on the TestCall contract
    function callFoo(address _test) external payable {
        // Using abi.encodeWithSignature to specify the function and arguments
        // 'value' sends ether, 'gas' limits the gas for this specific call
        (bool success, bytes memory _data) = _test.call{value: 111, gas: 5000}(
            abi.encodeWithSignature(
                "foo(string,uint256)", "call foo", 123
            )
        );

        require(success, "call failed");
        data = _data;
    }

    // Function to call a non-existent function to trigger fallback
    function callDoesNotExists(address _test) external {
        (bool success, ) = _test.call(
            abi.encodeWithSignature("doesNotExist()")
        );

        require(success, "call failed");
    }
}

