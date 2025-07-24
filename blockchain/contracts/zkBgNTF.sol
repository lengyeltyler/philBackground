// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title zkBgNFT
 * @dev NFT contract for ZK-verified spiral background patterns
 */
contract zkBgNFT is ERC721, ERC721URIStorage, Ownable {
    uint256 private _nextTokenId;
    // Struct to store spiral configuration
    struct SpiralConfig {
        uint64 seed;
        uint64 variant;
        uint8 spiralType;      // 0: Tight, 1: Loose, 2: Classic
        uint8 numArms;         // 3-8 arms
        uint64 spiralQuotient;
        uint64 armsQuotient;
        uint64 armsRemainder;
    }

    // Struct to store triangle data (simplified for on-chain storage)
    struct TriangleData {
        uint16 x1;
        uint16 y1;
        uint16 x2;
        uint16 y2;
        uint16 x3;
        uint16 y3;
        uint8 armIndex;
        uint8 triangleIndex;
    }

    // Mapping from token ID to spiral configuration
    mapping(uint256 => SpiralConfig) public spiralConfigs;
    
    // Mapping from token ID to triangle data array
    mapping(uint256 => TriangleData[]) public triangleData;
    
    // Mapping from token ID to ZK proof hash (simplified)
    mapping(uint256 => bytes32) public zkProofHashes;
    
    // Events
    event SpiralMinted(
        uint256 indexed tokenId,
        address indexed minter,
        uint64 seed,
        uint8 spiralType,
        uint8 numArms
    );

    constructor() ERC721("zkBg Spirals", "ZKBG") Ownable(msg.sender) {}

    /**
     * @dev Mint a new zkBg NFT with verified spiral data
     * @param to Address to mint to
     * @param config Spiral configuration from ZK circuit
     * @param triangles Triangle data (limited to save gas)
     * @param zkProofHash Hash of the ZK proof
     * @param uri Metadata URI (can be IPFS or on-chain SVG)
     */
    function mintSpiral(
        address to,
        SpiralConfig memory config,
        TriangleData[] memory triangles,
        bytes32 zkProofHash,
        string memory uri
    ) public returns (uint256) {
        // Validate configuration
        require(config.spiralType <= 2, "Invalid spiral type");
        require(config.numArms >= 3 && config.numArms <= 8, "Invalid number of arms");
        require(triangles.length > 0, "Invalid triangle count");
        
        // Get next token ID
        uint256 tokenId = _nextTokenId++;
        
        // ACTUALLY MINT THE NFT (this was missing!)
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
        
        // Store configuration
        spiralConfigs[tokenId] = config;
        zkProofHashes[tokenId] = zkProofHash;
        
        // Store triangle data
        for (uint i = 0; i < triangles.length; i++) {
            triangleData[tokenId].push(triangles[i]);
        }
        
        // Emit event
        emit SpiralMinted(tokenId, to, config.seed, config.spiralType, config.numArms);
        
        return tokenId;
    }

    /**
     * @dev Get spiral configuration for a token
     */
    function getSpiralConfig(uint256 tokenId) public view returns (SpiralConfig memory) {
        require(_exists(tokenId), "Token does not exist");
        return spiralConfigs[tokenId];
    }

    /**
     * @dev Get triangle count for a token
     */
    function getTriangleCount(uint256 tokenId) public view returns (uint256) {
        require(_exists(tokenId), "Token does not exist");
        return triangleData[tokenId].length;
    }

    /**
     * @dev Get triangle data for a token
     */
    function getTriangleData(uint256 tokenId, uint256 index) public view returns (TriangleData memory) {
        require(_exists(tokenId), "Token does not exist");
        require(index < triangleData[tokenId].length, "Triangle index out of bounds");
        return triangleData[tokenId][index];
    }

    /**
     * @dev Check if a token exists
     */
    function _exists(uint256 tokenId) internal view returns (bool) {
        return _ownerOf(tokenId) != address(0);
    }

    // Override required functions
    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}