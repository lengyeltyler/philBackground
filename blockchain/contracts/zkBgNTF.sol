// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title zkBgNFT
 * @dev Enhanced NFT contract for ZK-verified galaxy spiral background patterns
 * @notice Phase 1: Supports galaxy-density micro-triangles with gas optimization
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

    // Enhanced struct for galaxy triangle data
    struct TriangleData {
        uint16 x1; uint16 y1;
        uint16 x2; uint16 y2;
        uint16 x3; uint16 y3;
        uint8 armIndex;
        uint8 triangleIndex;
        uint8 triangleType;    // 0=spiral, 1=star, 2=core, 3=dust
        uint8 opacity;         // 0-100 for fading effects
    }

    // NEW: Galaxy-specific metadata
    struct GalaxyMetadata {
        uint16 particlesPerArm;    // 69 for galaxy density
        uint16 totalTriangles;     // Total micro-triangles
        uint8 galaxyType;          // 0=tight, 1=loose, 2=classic
        uint8 complexityLevel;     // 1-10 complexity rating
        bool isComplete;           // Flag for batch completion
    }

    // Storage mappings
    mapping(uint256 => SpiralConfig) public spiralConfigs;
    mapping(uint256 => TriangleData[]) public triangleData;
    mapping(uint256 => bytes32) public zkProofHashes;
    mapping(uint256 => GalaxyMetadata) public galaxyMetadata; // NEW
    
    // NEW: Batch storage for gas optimization
    mapping(uint256 => uint256) public triangleBatchCount;
    mapping(uint256 => bool) public mintingComplete;
    
    // Constants for galaxy validation
    uint16 public constant MAX_TRIANGLES_PER_BATCH = 690;
    uint16 public constant GALAXY_PARTICLES_PER_ARM = 69;
    uint16 public constant MAX_GALAXY_TRIANGLES = 8 * GALAXY_PARTICLES_PER_ARM; // 8 arms max
    
    // Events
    event SpiralMinted(
        uint256 indexed tokenId,
        address indexed minter,
        uint64 seed,
        uint8 spiralType,
        uint8 numArms
    );
    
    // NEW: Galaxy-specific events
    event GalaxyInitialized(
        uint256 indexed tokenId,
        uint16 expectedTriangles,
        uint8 galaxyType
    );
    
    event TriangleBatchAdded(
        uint256 indexed tokenId,
        uint256 batchNumber,
        uint256 trianglesAdded,
        bool isComplete
    );

    constructor() ERC721("zkBg Galaxy Spirals", "ZKBG") Ownable(msg.sender) {}

    /**
     * @dev Initialize a new zkBg Galaxy NFT (Step 1 of 2-step minting)
     * @param to Address to mint to
     * @param config Spiral configuration from ZK circuit
     * @param zkProofHash Hash of the ZK proof
     * @param uri Metadata URI
     * @param expectedTriangles Total triangles that will be added
     */
    function initializeGalaxy(
        address to,
        SpiralConfig memory config,
        bytes32 zkProofHash,
        string memory uri,
        uint16 expectedTriangles
    ) public returns (uint256) {
        // Validate configuration
        require(config.spiralType <= 2, "Invalid spiral type");
        require(config.numArms >= 3 && config.numArms <= 8, "Invalid number of arms");
        require(expectedTriangles > 0 && expectedTriangles <= MAX_GALAXY_TRIANGLES, "Invalid triangle count");
        
        // Get next token ID
        uint256 tokenId = _nextTokenId++;
        
        // Mint the NFT
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
        
        // Store configuration
        spiralConfigs[tokenId] = config;
        zkProofHashes[tokenId] = zkProofHash;
        
        // Initialize galaxy metadata
        galaxyMetadata[tokenId] = GalaxyMetadata({
            particlesPerArm: GALAXY_PARTICLES_PER_ARM,
            totalTriangles: expectedTriangles,
            galaxyType: config.spiralType,
            complexityLevel: _calculateComplexityLevel(config.numArms, expectedTriangles),
            isComplete: false
        });
        
        // Initialize batch tracking
        triangleBatchCount[tokenId] = 0;
        mintingComplete[tokenId] = false;
        
        // Emit events
        emit SpiralMinted(tokenId, to, config.seed, config.spiralType, config.numArms);
        emit GalaxyInitialized(tokenId, expectedTriangles, config.spiralType);
        
        return tokenId;
    }

    /**
     * @dev Add a batch of triangles to an existing galaxy (Step 2 of 2-step minting)
     * @param tokenId Token to add triangles to
     * @param triangles Batch of triangle data (max 100 per call)
     * @param isLastBatch True if this completes the galaxy
     */
    function addTriangleBatch(
        uint256 tokenId,
        TriangleData[] memory triangles,
        bool isLastBatch
    ) public {
        require(_exists(tokenId), "Token does not exist");
        require(!mintingComplete[tokenId], "Galaxy minting already complete");
        require(triangles.length > 0 && triangles.length <= MAX_TRIANGLES_PER_BATCH, "Invalid batch size");
        require(msg.sender == ownerOf(tokenId) || msg.sender == owner(), "Not authorized");
        
        GalaxyMetadata storage metadata = galaxyMetadata[tokenId];
        uint256 currentCount = triangleData[tokenId].length;
        
        // Validate we don't exceed expected triangle count
        require(currentCount + triangles.length <= metadata.totalTriangles, "Exceeds expected triangle count");
        
        // Add triangles to storage
        for (uint i = 0; i < triangles.length; i++) {
            TriangleData memory triangle = triangles[i];
            
            // Validate triangle data
            require(_isValidTriangle(triangle), "Invalid triangle data");
            
            triangleData[tokenId].push(triangle);
        }
        
        // Update batch tracking
        triangleBatchCount[tokenId]++;
        
        // Complete minting if this is the last batch
        if (isLastBatch) {
            require(triangleData[tokenId].length == metadata.totalTriangles, "Triangle count mismatch");
            mintingComplete[tokenId] = true;
            metadata.isComplete = true;
        }
        
        emit TriangleBatchAdded(
            tokenId,
            triangleBatchCount[tokenId],
            triangles.length,
            isLastBatch
        );
    }

    /**
     * @dev Legacy single-transaction mint for smaller spirals (backwards compatibility)
     */
    function mintSpiral(
        address to,
        SpiralConfig memory config,
        TriangleData[] memory triangles,
        bytes32 zkProofHash,
        string memory uri
    ) public returns (uint256) {
        require(triangles.length <= MAX_TRIANGLES_PER_BATCH, "Use batch minting for large galaxies");
        
        // Initialize galaxy
        uint256 tokenId = initializeGalaxy(to, config, zkProofHash, uri, uint16(triangles.length));
        
        // Add all triangles in single batch
        addTriangleBatch(tokenId, triangles, true);
        
        return tokenId;
    }

    /**
     * @dev Get galaxy metadata
     */
    function getGalaxyMetadata(uint256 tokenId) public view returns (GalaxyMetadata memory) {
        require(_exists(tokenId), "Token does not exist");
        return galaxyMetadata[tokenId];
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
     * @dev Get triangles in batches for gas efficiency
     */
    function getTriangleBatch(
        uint256 tokenId,
        uint256 startIndex,
        uint256 count
    ) public view returns (TriangleData[] memory) {
        require(_exists(tokenId), "Token does not exist");
        require(startIndex < triangleData[tokenId].length, "Start index out of bounds");
        
        uint256 endIndex = startIndex + count;
        if (endIndex > triangleData[tokenId].length) {
            endIndex = triangleData[tokenId].length;
        }
        
        TriangleData[] memory batch = new TriangleData[](endIndex - startIndex);
        for (uint256 i = 0; i < batch.length; i++) {
            batch[i] = triangleData[tokenId][startIndex + i];
        }
        
        return batch;
    }

    /**
     * @dev Check if galaxy minting is complete
     */
    function isGalaxyComplete(uint256 tokenId) public view returns (bool) {
        require(_exists(tokenId), "Token does not exist");
        return mintingComplete[tokenId];
    }

    /**
     * @dev Internal function to validate triangle data
     */
    function _isValidTriangle(TriangleData memory triangle) internal pure returns (bool) {
        // Validate triangle type
        if (triangle.triangleType > 3) return false; // 0=spiral, 1=star, 2=core, 3=dust
        
        // Validate opacity
        if (triangle.opacity > 100) return false;
        
        // Validate coordinates are not all the same (degenerate triangle)
        if (triangle.x1 == triangle.x2 && triangle.x2 == triangle.x3 &&
            triangle.y1 == triangle.y2 && triangle.y2 == triangle.y3) {
            return false;
        }
        
        return true;
    }

    /**
     * @dev Calculate complexity level based on arms and triangles
     */
    function _calculateComplexityLevel(uint8 numArms, uint16 totalTriangles) internal pure returns (uint8) {
        uint256 complexity = (uint256(numArms) * totalTriangles) / 50;
        if (complexity > 10) complexity = 10;
        if (complexity < 1) complexity = 1;
        return uint8(complexity);
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