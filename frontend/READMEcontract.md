# OnClick Smart Contract Specification

## Overview

This document outlines the smart contract functions needed to implement the OnClick Web3 payment platform. The platform supports three user roles: **Creators**, **Businesses**, and **Crowdfunders**, each with specific functionality for receiving payments, managing content, and tracking transactions.

---

## Core Data Structures

### User Roles

```solidity
enum UserRole {
    Creator,      // Digital artists, content creators
    Business,     // E-commerce, service providers
    Crowdfunder    // Campaign organizers
}
```

### Page Information

```solidity
struct Page {
    address owner;
    string handle;
    UserRole role;
    string name;                    // Short name (on-chain for quick access)
    // IPFS: description, bannerUrl, avatarUrl, theme, socialLinks, customFields
    bytes32 metadataHash;           // IPFS CID hash for full metadata
    uint256 goal;                   // Funding goal (0 for businesses)
    uint256 raised;                 // Total amount raised
    uint256 supporters;             // Number of supporters
    bool isActive;
    uint256 createdAt;
}
```

### Product (Business)

```solidity
struct Product {
    uint256 id;
    address businessOwner;
    string name;                    // Short name (on-chain for quick access)
    uint256 price;                  // Price in wei or token amount
    // IPFS: description, imageUrl, additionalImages, category, tags, specifications
    bytes32 metadataHash;           // IPFS CID hash for full metadata
    bool isActive;
    uint256 totalSold;
    uint256 createdAt;
}
```

### Campaign Milestone (Crowdfunder)

```solidity
struct Milestone {
    string title;                   // Short title (on-chain)
    uint256 targetAmount;
    bool completed;
    uint256 completedAt;
    // IPFS: Full milestone details (description, images) stored in campaign metadata
}
```

### Transaction

```solidity
struct Transaction {
    uint256 id;
    address from;
    address to;
    uint256 amount;
    // IPFS: message, supporterName (if provided)
    bytes32 messageHash;            // IPFS CID hash for message (empty bytes32 if no message)
    UserRole recipientRole;
    uint256 productId;              // 0 if not a product purchase
    uint256 timestamp;
    bool isRefunded;
}
```

---

## Core Functions

### 1. User & Page Management

#### `registerUser(string memory handle, UserRole role, string memory name)`

- **Purpose**: Register a new user and create their page
- **Parameters**:
  - `handle`: Unique handle for the page (e.g., "sarahchen")
  - `role`: User role (Creator, Business, or Crowdfunder)
  - `name`: Display name
- **Requirements**:
  - Handle must be unique and available
  - Handle must be at least 3 characters
  - User must not already be registered
- **Returns**: Page ID
- **Events**: `UserRegistered(address indexed user, string handle, UserRole role)`

#### `checkHandleAvailability(string memory handle) view returns (bool)`

- **Purpose**: Check if a handle is available
- **Parameters**: `handle`: Handle to check
- **Returns**: `true` if available, `false` if taken

#### `updatePageInfo(string memory handle, string memory description, string memory bannerUrl, string memory avatarUrl)`

- **Purpose**: Update page information
- **Parameters**:
  - `handle`: User's handle
  - `description`: Page description
  - `bannerUrl`: Banner image URL
  - `avatarUrl`: Avatar image URL
- **Requirements**: Only page owner can update
- **Events**: `PageUpdated(address indexed owner, string handle)`

#### `setFundingGoal(string memory handle, uint256 goal)`

- **Purpose**: Set or update funding goal (for Creators and Crowdfunders)
- **Parameters**:
  - `handle`: User's handle
  - `goal`: Funding goal in wei
- **Requirements**: Only page owner, role must be Creator or Crowdfunder
- **Events**: `GoalUpdated(address indexed owner, uint256 goal)`

#### `getPageByHandle(string memory handle) view returns (Page memory)`

- **Purpose**: Retrieve page information by handle
- **Parameters**: `handle`: Handle to query
- **Returns**: Page struct

#### `getPageByAddress(address user) view returns (Page memory)`

- **Purpose**: Retrieve page information by wallet address
- **Parameters**: `user`: Wallet address
- **Returns**: Page struct

---

### 2. Payment Functions

#### `makeDonation(string memory handle, string memory message) payable`

- **Purpose**: Make a donation to a Creator
- **Parameters**:
  - `handle`: Creator's handle
  - `message`: Optional message from supporter
- **Requirements**:
  - Recipient must be a Creator
  - Amount must be > 0
  - Page must be active
- **Actions**:
  - Transfer funds to creator (minus platform fee)
  - Update raised amount and supporter count
  - Record transaction
- **Events**: `DonationMade(address indexed from, address indexed to, uint256 amount, string message)`

#### `purchaseProduct(string memory handle, uint256 productId) payable`

- **Purpose**: Purchase a product from a Business
- **Parameters**:
  - `handle`: Business handle
  - `productId`: Product ID to purchase
- **Requirements**:
  - Recipient must be a Business
  - Product must exist and be active
  - Payment amount must match product price
- **Actions**:
  - Transfer funds to business (minus platform fee)
  - Update product sales count
  - Record transaction
- **Events**: `ProductPurchased(address indexed buyer, address indexed seller, uint256 productId, uint256 amount)`

#### `contributeToCampaign(string memory handle, string memory message) payable`

- **Purpose**: Contribute to a Crowdfunder campaign
- **Parameters**:
  - `handle`: Crowdfunder's handle
  - `message`: Optional message from supporter
- **Requirements**:
  - Recipient must be a Crowdfunder
  - Amount must be > 0
  - Campaign must be active
- **Actions**:
  - Transfer funds to campaign (minus platform fee)
  - Update raised amount and supporter count
  - Check and update milestone completion
  - Record transaction
- **Events**: `CampaignContribution(address indexed from, address indexed to, uint256 amount, string message)`

#### `withdrawFunds(string memory handle, uint256 amount)`

- **Purpose**: Withdraw funds from page (for page owners)
- **Parameters**:
  - `handle`: User's handle
  - `amount`: Amount to withdraw
- **Requirements**:
  - Only page owner can withdraw
  - Amount must be <= available balance
  - For Crowdfunders: may have withdrawal restrictions based on campaign status
- **Events**: `FundsWithdrawn(address indexed owner, uint256 amount)`

#### `getPageBalance(string memory handle) view returns (uint256)`

- **Purpose**: Get current balance for a page
- **Parameters**: `handle`: User's handle
- **Returns**: Balance in wei

---

### 3. Product Management (Business)

#### `createProduct(string memory handle, string memory name, string memory description, string memory imageUrl, uint256 price)`

- **Purpose**: Create a new product (Business only)
- **Parameters**:
  - `handle`: Business handle
  - `name`: Product name
  - `description`: Product description
  - `imageUrl`: Product image URL
  - `price`: Product price in wei
- **Requirements**:
  - Only business owner can create
  - User role must be Business
- **Returns**: Product ID
- **Events**: `ProductCreated(address indexed owner, uint256 productId, string name, uint256 price)`

#### `updateProduct(string memory handle, uint256 productId, string memory name, string memory description, string memory imageUrl, uint256 price)`

- **Purpose**: Update product information
- **Parameters**:
  - `handle`: Business handle
  - `productId`: Product ID
  - `name`: Updated name
  - `description`: Updated description
  - `imageUrl`: Updated image URL
  - `price`: Updated price
- **Requirements**: Only product owner can update
- **Events**: `ProductUpdated(address indexed owner, uint256 productId)`

#### `deleteProduct(string memory handle, uint256 productId)`

- **Purpose**: Delete/deactivate a product
- **Parameters**:
  - `handle`: Business handle
  - `productId`: Product ID
- **Requirements**: Only product owner can delete
- **Events**: `ProductDeleted(address indexed owner, uint256 productId)`

#### `getProductsByBusiness(string memory handle) view returns (Product[] memory)`

- **Purpose**: Get all products for a business
- **Parameters**: `handle`: Business handle
- **Returns**: Array of Product structs

#### `getProduct(uint256 productId) view returns (Product memory)`

- **Purpose**: Get product details by ID
- **Parameters**: `productId`: Product ID
- **Returns**: Product struct

---

### 4. Campaign Management (Crowdfunder)

#### `createCampaign(string memory handle, string memory title, string memory story, uint256 goal, uint256 deadline)`

- **Purpose**: Create or update campaign details
- **Parameters**:
  - `handle`: Crowdfunder handle
  - `title`: Campaign title
  - `story`: Campaign story/description
  - `goal`: Funding goal in wei
  - `deadline`: Campaign deadline (timestamp)
- **Requirements**:
  - Only crowdfunder owner can create/update
  - User role must be Crowdfunder
- **Events**: `CampaignCreated(address indexed owner, string title, uint256 goal)`

#### `addMilestone(string memory handle, string memory title, uint256 targetAmount)`

- **Purpose**: Add a milestone to a campaign
- **Parameters**:
  - `handle`: Crowdfunder handle
  - `title`: Milestone title
  - `targetAmount`: Target amount for milestone
- **Requirements**: Only campaign owner can add milestones
- **Events**: `MilestoneAdded(address indexed owner, string title, uint256 targetAmount)`

#### `updateMilestoneStatus(string memory handle, uint256 milestoneIndex)`

- **Purpose**: Mark milestone as completed when target is reached
- **Parameters**:
  - `handle`: Crowdfunder handle
  - `milestoneIndex`: Index of milestone
- **Requirements**: Automatically called when contribution reaches milestone target
- **Events**: `MilestoneCompleted(address indexed owner, uint256 milestoneIndex)`

#### `getCampaignMilestones(string memory handle) view returns (Milestone[] memory)`

- **Purpose**: Get all milestones for a campaign
- **Parameters**: `handle`: Crowdfunder handle
- **Returns**: Array of Milestone structs

#### `getCampaignProgress(string memory handle) view returns (uint256 raised, uint256 goal, uint256 percentage)`

- **Purpose**: Get campaign progress statistics
- **Parameters**: `handle`: Crowdfunder handle
- **Returns**: Raised amount, goal, and percentage

---

### 5. Transaction History

#### `getTransactionsByPage(string memory handle) view returns (Transaction[] memory)`

- **Purpose**: Get all transactions for a page
- **Parameters**: `handle`: User's handle
- **Returns**: Array of Transaction structs

#### `getTransactionsByUser(address user) view returns (Transaction[] memory)`

- **Purpose**: Get all transactions made by a user
- **Parameters**: `user`: Wallet address
- **Returns**: Array of Transaction structs

#### `getTransaction(uint256 transactionId) view returns (Transaction memory)`

- **Purpose**: Get specific transaction details
- **Parameters**: `transactionId`: Transaction ID
- **Returns**: Transaction struct

#### `getRecentTransactions(string memory handle, uint256 limit) view returns (Transaction[] memory)`

- **Purpose**: Get recent transactions for a page (for display)
- **Parameters**:
  - `handle`: User's handle
  - `limit`: Maximum number of transactions to return
- **Returns**: Array of recent Transaction structs

---

### 6. Platform Management

#### `setPlatformFee(uint256 feePercentage)`

- **Purpose**: Set platform fee percentage (admin only)
- **Parameters**: `feePercentage`: Fee percentage (e.g., 250 = 2.5%)
- **Requirements**: Only contract owner/admin
- **Events**: `PlatformFeeUpdated(uint256 newFee)`

#### `withdrawPlatformFees()`

- **Purpose**: Withdraw accumulated platform fees (admin only)
- **Requirements**: Only contract owner/admin
- **Events**: `PlatformFeesWithdrawn(uint256 amount)`

#### `pauseContract()`

- **Purpose**: Pause all contract functions (emergency)
- **Requirements**: Only contract owner/admin
- **Events**: `ContractPaused()`

#### `unpauseContract()`

- **Purpose**: Unpause contract functions
- **Requirements**: Only contract owner/admin
- **Events**: `ContractUnpaused()`

---

### 7. Statistics & Analytics

#### `getPageStats(string memory handle) view returns (uint256 totalRaised, uint256 totalSupporters, uint256 totalTransactions)`

- **Purpose**: Get statistics for a page
- **Parameters**: `handle`: User's handle
- **Returns**: Statistics struct

#### `getBusinessStats(string memory handle) view returns (uint256 totalRevenue, uint256 totalOrders, uint256 activeProducts)`

- **Purpose**: Get business-specific statistics
- **Parameters**: `handle`: Business handle
- **Returns**: Business statistics

#### `getCampaignStats(string memory handle) view returns (uint256 raised, uint256 goal, uint256 supporters, uint256 daysLeft)`

- **Purpose**: Get campaign-specific statistics
- **Parameters**: `handle`: Crowdfunder handle
- **Returns**: Campaign statistics

---

## Events

All major actions should emit events for frontend integration and indexing:

```solidity
event UserRegistered(address indexed user, string handle, UserRole role);
event PageUpdated(address indexed owner, string handle);
event DonationMade(address indexed from, address indexed to, uint256 amount, string message);
event ProductPurchased(address indexed buyer, address indexed seller, uint256 productId, uint256 amount);
event CampaignContribution(address indexed from, address indexed to, uint256 amount, string message);
event ProductCreated(address indexed owner, uint256 productId, string name, uint256 price);
event ProductUpdated(address indexed owner, uint256 productId);
event MilestoneCompleted(address indexed owner, uint256 milestoneIndex);
event FundsWithdrawn(address indexed owner, uint256 amount);
event PlatformFeeUpdated(uint256 newFee);
```

---

## IPFS Storage Strategy

To optimize gas costs and enable decentralized storage, certain data should be stored on IPFS with only the IPFS hash (CID) stored on-chain. This section outlines what should be stored on IPFS and how to structure the metadata.

### Data to Store on IPFS

#### 1. **Page Metadata** (`PageMetadata.json`)

Store comprehensive page information that doesn't need on-chain verification:

```json
{
  "name": "Sarah Chen",
  "description": "Creating beautiful digital art and helping other artists grow their skills...",
  "bannerUrl": "ipfs://QmXXX...",
  "avatarUrl": "ipfs://QmYYY...",
  "theme": "#8CCDEB",
  "socialLinks": {
    "twitter": "https://twitter.com/...",
    "instagram": "https://instagram.com/...",
    "website": "https://..."
  },
  "customFields": {}
}
```

**On-chain**: Store only the IPFS CID hash (bytes32 or string)

#### 2. **Product Metadata** (`ProductMetadata.json`)

Store product details and images:

```json
{
  "name": "Digital Art License",
  "description": "Commercial license for digital artwork...",
  "imageUrl": "ipfs://QmXXX...",
  "additionalImages": ["ipfs://QmYYY...", "ipfs://QmZZZ..."],
  "category": "Digital License",
  "tags": ["art", "license", "commercial"],
  "specifications": {}
}
```

**On-chain**: Store only essential data (price, active status) + IPFS CID

#### 3. **Campaign Metadata** (`CampaignMetadata.json`)

Store campaign story and media:

```json
{
  "title": "Green Blockchain Revolution",
  "story": "We're developing innovative consensus mechanisms...",
  "bannerUrl": "ipfs://QmXXX...",
  "videoUrl": "ipfs://QmYYY...",
  "milestones": [
    {
      "title": "Research Phase",
      "description": "...",
      "imageUrl": "ipfs://QmZZZ..."
    }
  ],
  "updates": []
}
```

**On-chain**: Store goal, deadline, milestone targets + IPFS CID

#### 4. **Transaction Messages** (`TransactionMessage.json`)

Store supporter messages (optional, for privacy):

```json
{
  "message": "Love your work! Keep creating!",
  "supporterName": "CryptoFan123",
  "isPublic": true
}
```

**On-chain**: Store only IPFS CID if message exists, or empty string

#### 5. **User Profile Extended Data** (`ProfileExtended.json`)

Store additional profile information:

```json
{
  "bio": "Full biography text...",
  "location": "San Francisco, CA",
  "skills": ["Digital Art", "NFT Creation"],
  "portfolio": ["ipfs://QmXXX...", "ipfs://QmYYY..."],
  "achievements": []
}
```

### Updated Data Structures with IPFS

```solidity
struct Page {
    address owner;
    string handle;
    UserRole role;
    string name;                    // Short name (on-chain)
    // IPFS: description, bannerUrl, avatarUrl, theme, socialLinks, customFields
    bytes32 metadataHash;           // IPFS CID for full metadata
    uint256 goal;
    uint256 raised;
    uint256 supporters;
    bool isActive;
    uint256 createdAt;
}

struct Product {
    uint256 id;
    address businessOwner;
    string name;                    // Short name (on-chain)
    uint256 price;
    // IPFS: description, imageUrl, additionalImages, category, tags, specifications
    bytes32 metadataHash;           // IPFS CID for full metadata
    bool isActive;
    uint256 totalSold;
    uint256 createdAt;
}

struct Campaign {
    address owner;
    string handle;
    uint256 goal;
    uint256 deadline;
    // IPFS: title, story, bannerUrl, videoUrl, milestones (with descriptions/images), updates
    bytes32 metadataHash;           // IPFS CID for campaign story, milestones
    uint256 raised;
    uint256 supporters;
    bool isActive;
}
```

### IPFS Integration Functions

#### `updatePageMetadata(string memory handle, bytes32 ipfsHash)`

- **Purpose**: Update page metadata stored on IPFS
- **Parameters**:
  - `handle`: User's handle
  - `ipfsHash`: IPFS CID hash of the metadata JSON
- **Requirements**: Only page owner
- **Events**: `PageMetadataUpdated(address indexed owner, bytes32 ipfsHash)`

#### `updateProductMetadata(string memory handle, uint256 productId, bytes32 ipfsHash)`

- **Purpose**: Update product metadata on IPFS
- **Parameters**:
  - `handle`: Business handle
  - `productId`: Product ID
  - `ipfsHash`: IPFS CID hash
- **Events**: `ProductMetadataUpdated(uint256 productId, bytes32 ipfsHash)`

#### `getPageMetadata(string memory handle) view returns (bytes32)`

- **Purpose**: Get IPFS hash for page metadata
- **Returns**: IPFS CID hash
- **Frontend**: Use this hash to fetch JSON from IPFS gateway

### IPFS Gateway Access Pattern

```javascript
// Frontend example
const ipfsHash = await contract.getPageMetadata(handle);
const metadataUrl = `https://ipfs.io/ipfs/${ipfsHash}`;
const response = await fetch(metadataUrl);
const metadata = await response.json();
```

### Recommended IPFS Structure

```
ipfs://
├── pages/
│   ├── {handle}/
│   │   ├── metadata.json
│   │   ├── banner.jpg
│   │   └── avatar.jpg
├── products/
│   ├── {businessHandle}/
│   │   ├── {productId}/
│   │   │   ├── metadata.json
│   │   │   └── images/
│   │   │       ├── main.jpg
│   │   │       └── gallery/
├── campaigns/
│   ├── {handle}/
│   │   ├── metadata.json
│   │   ├── banner.jpg
│   │   └── milestones/
└── transactions/
    └── {transactionId}/
        └── message.json
```

### IPFS Pinning Strategy

1. **User Responsibility**: Users pin their own content
2. **Platform Pinning Service**: Optional service to pin popular content
3. **IPFS Providers**: Consider using Pinata, NFT.Storage, or Web3.Storage
4. **Redundancy**: Encourage multiple pinning services for important content

### Gas Cost Comparison

**Storing on-chain (example)**:

- Full description (200 chars): ~40,000 gas
- Image URL (100 chars): ~20,000 gas
- Total: ~60,000 gas per update

**Storing on IPFS**:

- IPFS hash (32 bytes): ~20,000 gas
- **Savings: ~66% gas reduction**

### Migration Strategy

1. **Phase 1**: Store new data on IPFS, keep old string fields for backward compatibility
2. **Phase 2**: Migrate existing data to IPFS
3. **Phase 3**: Remove string fields, use only IPFS hashes

### IPFS Metadata Schema Standards

Use JSON Schema for validation:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "name": { "type": "string", "maxLength": 100 },
    "description": { "type": "string", "maxLength": 5000 },
    "bannerUrl": { "type": "string", "pattern": "^ipfs://" },
    "avatarUrl": { "type": "string", "pattern": "^ipfs://" }
  },
  "required": ["name", "description"]
}
```

### Security Considerations for IPFS

1. **Content Validation**: Validate JSON schema before accepting IPFS hash
2. **Hash Verification**: Verify IPFS hash format before storing
3. **Pinning Verification**: Optionally verify content is pinned before accepting
4. **Gateway Redundancy**: Support multiple IPFS gateways for reliability
5. **Content Moderation**: Consider content moderation for public-facing data

---

## Security Considerations

1. **Access Control**: All state-changing functions must verify ownership
2. **Reentrancy Protection**: Use ReentrancyGuard for payment functions
3. **Input Validation**: Validate all string inputs (handle format, length)
4. **Overflow Protection**: Use SafeMath or Solidity 0.8+ built-in checks
5. **Pausable**: Implement emergency pause functionality
6. **Handle Uniqueness**: Enforce unique handles with proper checks
7. **Payment Verification**: Ensure payment amounts match expected values
8. **Withdrawal Limits**: Implement appropriate withdrawal restrictions for campaigns

---

## Gas Optimization

1. **Batch Operations**: Consider batch functions for multiple updates
2. **Storage Optimization**: Pack structs efficiently
3. **Event Indexing**: Use indexed parameters for efficient filtering
4. **View Functions**: Use view functions for read-only operations
5. **Mapping vs Arrays**: Use mappings for O(1) lookups where possible

---

## Implementation Notes

1. **Token Support**: Consider ERC20 token support in addition to native currency
2. **Multi-chain**: Design for potential multi-chain deployment
3. **Upgradeability**: Consider proxy pattern for future upgrades
4. **IPFS Integration**: See detailed IPFS Storage Strategy section above
5. **Oracle Integration**: May need price oracles for fiat-to-crypto conversions
6. **IPFS Pinning**: Implement or recommend IPFS pinning service for content persistence

---

## Testing Checklist

- [ ] User registration and handle uniqueness
- [ ] Payment functions for all three roles
- [ ] Product creation and purchase flow
- [ ] Campaign creation and milestone tracking
- [ ] Withdrawal functionality
- [ ] Access control and ownership verification
- [ ] Edge cases (zero amounts, invalid handles, etc.)
- [ ] Gas optimization
- [ ] Reentrancy protection
- [ ] Event emission

---

## Future Enhancements

1. **Multi-token Support**: Accept multiple ERC20 tokens
2. **Subscription Payments**: Recurring payment support
3. **NFT Integration**: Mint NFTs for supporters
4. **Governance**: DAO governance for platform decisions
5. **Staking**: Staking mechanisms for platform users
6. **Referral System**: Referral rewards program
7. **Escrow Services**: Escrow for product purchases
8. **Dispute Resolution**: Dispute handling mechanism

---

**Document Version**: 1.0  
**Last Updated**: 2024
