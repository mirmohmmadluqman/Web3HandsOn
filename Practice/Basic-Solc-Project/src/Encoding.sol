// SPDX-License-Identifier: MIT

pragma solidity ^0.8.7;

contract Encoding {
    function combineStrings() public pure returns (string memory) {
        return string(abi.encodePacked("Hi Mom!"," ", "Miss you!"));
    }

    function notCombineString() public pure returns (bytes memory) {
        bytes memory someString = abi.encodePacked("Hi Mom!"," ", "Miss you!");
        return someString;
    }

    function encodeNumber() public pure returns (bytes memory) {
        bytes memory number = abi.encode(1);
        return number;
    }

    function encodeString() public pure returns (bytes memory) {
        bytes memory someString = abi.encode("Hello");
        return someString;
    }

    function encodePackedString() public pure returns (bytes memory) {
        bytes memory someString = abi.encodePacked("Something...");
        return someString;
    }

    function mutliEncode() public pure returns (bytes memory) {
        bytes memory someString = abi.encode("some string", " it's much small", " Cmon");
        return someString;
    }


    function multiDecode() public pure returns (string memory, string memory, string memory) {
        (string memory someString, string memory theString, string memory notString) = abi.decode(mutliEncode(), (string, string, string));
        return (someString, theString, notString);
    }
    
}