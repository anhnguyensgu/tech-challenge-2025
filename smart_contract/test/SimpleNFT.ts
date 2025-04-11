const { expect } = require("chai");

describe("Simple NFT contract", function () {
  it("Deployment should assign the total supply of tokens to the owner", async function () {
    //const [owner] = await ethers.getSigners();

    const hardhatToken = await ethers.deployContract("SimpleNFT");

    const [owner, user1, user2] = await ethers.getSigners();
    const tokenId = 1;
    await hardhatToken.connect(user1).mint(tokenId);
    await hardhatToken.connect(user2).mint(2);
    const ownerBalance = await hardhatToken.ownerOf(tokenId);
    expect(ownerBalance).to.equal(user1.address);

    await hardhatToken.connect(user1).transfer(user2.address, tokenId);

    const newOwner = await hardhatToken.ownerOf(tokenId);
    const balanceOf = await hardhatToken.balanceOf(user2.address);
    console.log("balanceOf", balanceOf);
    expect(newOwner).to.equal(user2.address);
  });
});
