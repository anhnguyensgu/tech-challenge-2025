// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SimpleNFT is ERC721, Ownable {
    constructor() ERC721("SimpleNFT", "SNFT") Ownable(msg.sender) {} 
    function mint(uint256 tokenId) public returns (uint256) {
        _safeMint(msg.sender, tokenId);
        return tokenId;
    }

    function transfer(address to, uint256 tokenId) public {
        require(ownerOf(tokenId) == msg.sender, "You do not own this token");
        _transfer(msg.sender, to, tokenId);
    }
}
