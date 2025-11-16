#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{aliases::U8, Address, FixedBytes, U256},
    crypto, evm,
    prelude::*,
};

sol! {
    enum UserRole {
        Creator,
        Business,
        Crowdfunder
    }

    // Events
    event UserRegistered(address indexed user, string handle, uint8 role);
    event PageUpdated(address indexed owner, string handle);
    event PageMetadataUpdated(address indexed owner, string handle, string metadataHash);
    event GoalUpdated(address indexed owner, uint256 goal);
    event DonationMade(address indexed from, address indexed to, uint256 amount, string messageHash);
    event ProductPurchased(address indexed buyer, address indexed seller, uint256 productId, uint256 amount);
    event CampaignContribution(address indexed from, address indexed to, uint256 amount, string messageHash);
    event ProductCreated(address indexed owner, uint256 productId, string name, uint256 price);
    event ProductUpdated(address indexed owner, uint256 productId, string metadataHash);
    event ProductDeleted(address indexed owner, uint256 productId);
    event CampaignCreated(address indexed owner, uint256 goal, string metadataHash);
    event MilestoneAdded(address indexed owner, string title, uint256 targetAmount);
    event MilestoneCompleted(address indexed owner, uint256 milestoneIndex);
    event FundsWithdrawn(address indexed owner, uint256 amount);
    event PlatformFeeUpdated(uint256 newFee);
    event PlatformFeesWithdrawn(uint256 amount);
    event PaymentIntentCreated(bytes32 indexed intentId, address indexed creator, string handle, uint256 amount);
    event PaymentIntentPaid(bytes32 indexed intentId, address indexed payer, uint256 amount);
    event PaymentIntentCancelled(bytes32 indexed intentId);

    // Errors
    error PageNotFound();
    error ProductNotFound();
    error MilestoneNotFound();
    error TransactionNotFound();
    error HandleAlreadyTaken();
    error InvalidHandle();
    error NotPageOwner();
    error InvalidRole();
    error InsufficientFunds();
    error InvalidAmount();
    error PageNotActive();
    error ProductNotActive();
    error CampaignNotActive();
    error TransferFailed();
    error PaymentIntentNotFound();
    error PaymentIntentExpired();
    error PaymentIntentInactive();
    error PaymentIntentMaxUsages();
    error InvalidExpiration();
}

#[derive(SolidityError)]
pub enum OnClickContractError {
    PageNotFound(PageNotFound),
    ProductNotFound(ProductNotFound),
    MilestoneNotFound(MilestoneNotFound),
    TransactionNotFound(TransactionNotFound),
    HandleAlreadyTaken(HandleAlreadyTaken),
    InvalidHandle(InvalidHandle),
    NotPageOwner(NotPageOwner),
    InvalidRole(InvalidRole),
    InsufficientFunds(InsufficientFunds),
    InvalidAmount(InvalidAmount),
    PageNotActive(PageNotActive),
    ProductNotActive(ProductNotActive),
    CampaignNotActive(CampaignNotActive),
    TransferFailed(TransferFailed),
    PaymentIntentNotFound(PaymentIntentNotFound),
    PaymentIntentExpired(PaymentIntentExpired),
    PaymentIntentInactive(PaymentIntentInactive),
    PaymentIntentMaxUsages(PaymentIntentMaxUsages),
    InvalidExpiration(InvalidExpiration),
}

impl From<Vec<u8>> for OnClickContractError {
    fn from(_: Vec<u8>) -> Self {
        OnClickContractError::TransferFailed(TransferFailed {})
    }
}

sol_storage! {
    pub struct Page {
        address owner;
        string handle;
        uint8 role;
        string name;
        string metadataHash;
        uint256 goal;
        uint256 raised;
        uint256 supporters;
        bool isActive;
        uint256 createdAt;
    }
}

sol_storage! {
    pub struct Product {
        uint256 id;
        address businessOwner;
        string name;
        uint256 price;
        string metadataHash;
        bool isActive;
        uint256 totalSold;
        uint256 createdAt;
    }
}

sol_storage! {
    pub struct Milestone {
        string title;
        uint256 targetAmount;
        bool completed;
        uint256 completedAt;
        string metadataHash;
    }
}

sol_storage! {
    pub struct Transaction {
        uint256 id;
        address from;
        address to;
        uint256 amount;
        string messageHash;
        uint8 recipientRole;
        uint256 productId;
        uint256 timestamp;
        bool isRefunded;
    }
}

sol_storage! {
    pub struct PaymentIntent {
        bytes32 id;
        address creator;
        string handle;
        uint256 amount;
        string description;
        bool isActive;
        uint256 createdAt;
        uint256 expiresAt;
        uint256 usageCount;
        uint256 maxUsages; // 0 = unlimited
    }
}

sol_storage! {
    #[entrypoint]
    pub struct OnClickContract {
        // Counters
        uint256 pageId;
        uint256 productId;
        uint256 transactionId;

        // Mappings
        mapping(address => Page) pages;
        mapping(string => address) handleToAddress;
        mapping(uint256 => Product) products;
        mapping(uint256 => Milestone) milestones;
        mapping(uint256 => Transaction) transactions;
        mapping(address => uint256[]) userProducts;
        mapping(address => uint256[]) userTransactions;
        mapping(string => uint256[]) pageTransactions;
        mapping(bytes32 => PaymentIntent) paymentIntents;
        mapping(string => bytes32[]) handlePaymentIntents;
        mapping(address => bytes32[]) userPaymentIntents;

        // Platform settings
        address owner;
        uint256 platformFee; // in basis points (e.g., 250 = 2.5%)
        uint256 platformFeesCollected;
        bool paused;
    }
}

#[public]
impl OnClickContract {
    // ==================== User & Page Management ====================

    /// Register a new user and create their page
    pub fn register_user(
        &mut self,
        handle: String,
        role: u8,
        name: String,
        metadata_hash: String,
    ) -> Result<U256, OnClickContractError> {
        // Validate handle
        if handle.len() < 3 {
            return Err(OnClickContractError::InvalidHandle(InvalidHandle {}));
        }

        // Check if handle is already taken
        let handle_addr = self.handleToAddress.getter(handle.clone());
        if handle_addr.get() != Address::ZERO {
            return Err(OnClickContractError::HandleAlreadyTaken(
                HandleAlreadyTaken {},
            ));
        }

        // Check if user already has a page
        let sender = self.vm().msg_sender();
        let existing_page = self.pages.getter(sender);
        if existing_page.owner.get() != Address::ZERO {
            return Err(OnClickContractError::HandleAlreadyTaken(
                HandleAlreadyTaken {},
            ));
        }

        // Validate role
        if role > 2 {
            return Err(OnClickContractError::InvalidRole(InvalidRole {}));
        }

        // Get next page ID
        let page_id = self.pageId.get();
        self.pageId.set(page_id + U256::from(1));

        // Capture timestamp before mutable borrow
        let timestamp = U256::from(self.vm().block_timestamp());

        // Create page
        let mut page = self.pages.setter(sender);
        page.owner.set(sender);
        page.handle.set_str(&handle);
        page.role.set(U8::from(role));
        page.name.set_str(&name);
        page.metadataHash.set_str(&metadata_hash);
        page.goal.set(U256::ZERO);
        page.raised.set(U256::ZERO);
        page.supporters.set(U256::ZERO);
        page.isActive.set(true);
        page.createdAt.set(timestamp);

        // Map handle to address
        self.handleToAddress.setter(handle.clone()).set(sender);

        // Emit event
        evm::log(UserRegistered {
            user: sender,
            handle: handle.clone(),
            role,
        });

        Ok(page_id)
    }

    /// Check if a handle is available
    pub fn check_handle_availability(&self, handle: String) -> bool {
        let handle_addr = self.handleToAddress.getter(handle);
        handle_addr.get() == Address::ZERO
    }

    /// Update page information
    pub fn update_page_info(
        &mut self,
        handle: String,
        name: String,
        metadata_hash: String,
    ) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if owner != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let mut page = self.pages.setter(owner);
        page.name.set_str(&name);
        page.metadataHash.set_str(&metadata_hash);

        evm::log(PageUpdated {
            owner,
            handle: handle.clone(),
        });

        evm::log(PageMetadataUpdated {
            owner,
            handle: handle.clone(),
            metadataHash: metadata_hash,
        });

        Ok(())
    }

    /// Set or update funding goal
    pub fn set_funding_goal(
        &mut self,
        handle: String,
        goal: U256,
    ) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if owner != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let mut page = self.pages.setter(owner);
        let role = page.role.get();

        // Only Creator (0) and Crowdfunder (2) can have goals
        if role != U8::from(0) && role != U8::from(2) {
            return Err(OnClickContractError::InvalidRole(InvalidRole {}));
        }

        page.goal.set(goal);

        evm::log(GoalUpdated { owner, goal });

        Ok(())
    }

    /// Get page by handle
    pub fn get_page_by_handle(
        &self,
        handle: String,
    ) -> Result<
        (
            Address,
            String,
            u8,
            String,
            String,
            U256,
            U256,
            U256,
            bool,
            U256,
        ),
        OnClickContractError,
    > {
        let page_addr = self.handleToAddress.getter(handle);
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        let page = self.pages.getter(owner);
        Ok((
            page.owner.get(),
            page.handle.get_string(),
            page.role.get().to::<u8>(),
            page.name.get_string(),
            page.metadataHash.get_string(),
            page.goal.get(),
            page.raised.get(),
            page.supporters.get(),
            page.isActive.get(),
            page.createdAt.get(),
        ))
    }

    /// Get page by address
    pub fn get_page_by_address(
        &self,
        user: Address,
    ) -> Result<
        (
            Address,
            String,
            u8,
            String,
            String,
            U256,
            U256,
            U256,
            bool,
            U256,
        ),
        OnClickContractError,
    > {
        let page = self.pages.getter(user);

        if page.owner.get() == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        Ok((
            page.owner.get(),
            page.handle.get_string(),
            page.role.get().to::<u8>(),
            page.name.get_string(),
            page.metadataHash.get_string(),
            page.goal.get(),
            page.raised.get(),
            page.supporters.get(),
            page.isActive.get(),
            page.createdAt.get(),
        ))
    }

    // ==================== Payment Functions ====================

    /// Make a donation to a Creator
    #[payable]
    pub fn make_donation(
        &mut self,
        handle: String,
        message_hash: String,
    ) -> Result<U256, OnClickContractError> {
        let amount = self.vm().msg_value();
        if amount == U256::ZERO {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let recipient = page_addr.get();

        if recipient == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        let mut page = self.pages.setter(recipient);

        if !page.isActive.get() {
            return Err(OnClickContractError::PageNotActive(PageNotActive {}));
        }

        if page.role.get() != U8::from(0) {
            // 0 = Creator
            return Err(OnClickContractError::InvalidRole(InvalidRole {}));
        }

        // Calculate platform fee
        let fee = (amount * self.platformFee.get()) / U256::from(10000);
        let net_amount = amount - fee;

        // Update page stats
        let raised = page.raised.get();
        page.raised.set(raised + net_amount);
        let supporters = page.supporters.get();
        page.supporters.set(supporters + U256::from(1));

        // Transfer funds
        self.vm().transfer_eth(recipient, net_amount)?;

        // Update platform fees
        let fees = self.platformFeesCollected.get();
        self.platformFeesCollected.set(fees + fee);

        // Create transaction
        let tx_id = self.transactionId.get();
        self.transactionId.set(tx_id + U256::from(1));
        let timestamp = U256::from(self.vm().block_timestamp());

        let mut transaction = self.transactions.setter(tx_id);
        transaction.id.set(tx_id);
        transaction.from.set(sender);
        transaction.to.set(recipient);
        transaction.amount.set(net_amount);
        transaction.messageHash.set_str(&message_hash);
        transaction.recipientRole.set(U8::from(0));
        transaction.productId.set(U256::ZERO);
        transaction.timestamp.set(timestamp);
        transaction.isRefunded.set(false);

        // Emit event
        evm::log(DonationMade {
            from: sender,
            to: recipient,
            amount: net_amount,
            messageHash: message_hash,
        });

        Ok(tx_id)
    }

    /// Purchase a product from a Business
    #[payable]
    pub fn purchase_product(
        &mut self,
        handle: String,
        product_id: U256,
    ) -> Result<U256, OnClickContractError> {
        let amount = self.vm().msg_value();
        if amount == U256::ZERO {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let business = page_addr.get();

        if business == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        let page = self.pages.getter(business);
        if page.role.get() != U8::from(1) {
            // 1 = Business
            return Err(OnClickContractError::InvalidRole(InvalidRole {}));
        }

        let mut product = self.products.setter(product_id);
        if product.businessOwner.get() == Address::ZERO {
            return Err(OnClickContractError::ProductNotFound(ProductNotFound {}));
        }

        if product.businessOwner.get() != business {
            return Err(OnClickContractError::ProductNotFound(ProductNotFound {}));
        }

        if !product.isActive.get() {
            return Err(OnClickContractError::ProductNotActive(ProductNotActive {}));
        }

        let price = product.price.get();
        if amount != price {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        // Calculate platform fee
        let fee = (amount * self.platformFee.get()) / U256::from(10000);
        let net_amount = amount - fee;

        // Update product sales
        let sold = product.totalSold.get();
        product.totalSold.set(sold + U256::from(1));

        // Transfer funds
        self.vm().transfer_eth(business, net_amount)?;

        // Update platform fees
        let fees = self.platformFeesCollected.get();
        self.platformFeesCollected.set(fees + fee);

        // Create transaction
        let tx_id = self.transactionId.get();
        self.transactionId.set(tx_id + U256::from(1));
        let timestamp = U256::from(self.vm().block_timestamp());

        let mut transaction = self.transactions.setter(tx_id);
        transaction.id.set(tx_id);
        transaction.from.set(sender);
        transaction.to.set(business);
        transaction.amount.set(net_amount);
        transaction.messageHash.set_str("");
        transaction.recipientRole.set(U8::from(1));
        transaction.productId.set(product_id);
        transaction.timestamp.set(timestamp);
        transaction.isRefunded.set(false);

        // Emit event
        evm::log(ProductPurchased {
            buyer: sender,
            seller: business,
            productId: product_id,
            amount: net_amount,
        });

        Ok(tx_id)
    }

    /// Contribute to a Crowdfunder campaign
    #[payable]
    pub fn contribute_to_campaign(
        &mut self,
        handle: String,
        message_hash: String,
    ) -> Result<U256, OnClickContractError> {
        let amount = self.vm().msg_value();
        if amount == U256::ZERO {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let recipient = page_addr.get();

        if recipient == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        let mut page = self.pages.setter(recipient);

        if !page.isActive.get() {
            return Err(OnClickContractError::CampaignNotActive(
                CampaignNotActive {},
            ));
        }

        if page.role.get() != U8::from(2) {
            // 2 = Crowdfunder
            return Err(OnClickContractError::InvalidRole(InvalidRole {}));
        }

        // Calculate platform fee
        let fee = (amount * self.platformFee.get()) / U256::from(10000);
        let net_amount = amount - fee;

        // Update page stats
        let raised = page.raised.get();
        page.raised.set(raised + net_amount);
        let supporters = page.supporters.get();
        page.supporters.set(supporters + U256::from(1));

        // Transfer funds (campaigns hold funds in contract)
        // Note: For campaigns, funds might be held until goal is reached
        // For now, we transfer directly
        self.vm().transfer_eth(recipient, net_amount)?;

        // Update platform fees
        let fees = self.platformFeesCollected.get();
        self.platformFeesCollected.set(fees + fee);

        // Create transaction
        let tx_id = self.transactionId.get();
        self.transactionId.set(tx_id + U256::from(1));
        let timestamp = U256::from(self.vm().block_timestamp());

        let mut transaction = self.transactions.setter(tx_id);
        transaction.id.set(tx_id);
        transaction.from.set(sender);
        transaction.to.set(recipient);
        transaction.amount.set(net_amount);
        transaction.messageHash.set_str(&message_hash);
        transaction.recipientRole.set(U8::from(2));
        transaction.productId.set(U256::ZERO);
        transaction.timestamp.set(timestamp);
        transaction.isRefunded.set(false);

        // Emit event
        evm::log(CampaignContribution {
            from: sender,
            to: recipient,
            amount: net_amount,
            messageHash: message_hash,
        });

        Ok(tx_id)
    }

    /// Withdraw funds from page
    pub fn withdraw_funds(
        &mut self,
        handle: String,
        amount: U256,
    ) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if owner != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let page = self.pages.getter(owner);
        let raised = page.raised.get();

        if amount > raised {
            return Err(OnClickContractError::InsufficientFunds(
                InsufficientFunds {},
            ));
        }

        // Update raised amount
        let mut page_mut = self.pages.setter(owner);
        page_mut.raised.set(raised - amount);

        // Transfer funds
        self.vm().transfer_eth(sender, amount)?;

        evm::log(FundsWithdrawn { owner, amount });

        Ok(())
    }

    /// Get page balance
    pub fn get_page_balance(&self, handle: String) -> Result<U256, OnClickContractError> {
        let page_addr = self.handleToAddress.getter(handle);
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        let page = self.pages.getter(owner);
        Ok(page.raised.get())
    }

    // ==================== Product Management ====================

    /// Create a new product
    pub fn create_product(
        &mut self,
        handle: String,
        name: String,
        price: U256,
        metadata_hash: String,
    ) -> Result<U256, OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if owner != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let page = self.pages.getter(owner);
        if page.role.get() != U8::from(1) {
            // 1 = Business
            return Err(OnClickContractError::InvalidRole(InvalidRole {}));
        }

        let product_id = self.productId.get();
        self.productId.set(product_id + U256::from(1));
        let timestamp = U256::from(self.vm().block_timestamp());

        let mut product = self.products.setter(product_id);
        product.id.set(product_id);
        product.businessOwner.set(owner);
        product.name.set_str(&name);
        product.price.set(price);
        product.metadataHash.set_str(&metadata_hash);
        product.isActive.set(true);
        product.totalSold.set(U256::ZERO);
        product.createdAt.set(timestamp);

        evm::log(ProductCreated {
            owner,
            productId: product_id,
            name: name.clone(),
            price,
        });

        Ok(product_id)
    }

    /// Update product information
    pub fn update_product(
        &mut self,
        handle: String,
        product_id: U256,
        name: String,
        price: U256,
        metadata_hash: String,
    ) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if owner != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let mut product = self.products.setter(product_id);
        if product.businessOwner.get() != owner {
            return Err(OnClickContractError::ProductNotFound(ProductNotFound {}));
        }

        product.name.set_str(&name);
        product.price.set(price);
        product.metadataHash.set_str(&metadata_hash);

        evm::log(ProductUpdated {
            owner,
            productId: product_id,
            metadataHash: metadata_hash,
        });

        Ok(())
    }

    /// Delete/deactivate a product
    pub fn delete_product(
        &mut self,
        handle: String,
        product_id: U256,
    ) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page_addr = self.handleToAddress.getter(handle.clone());
        let owner = page_addr.get();

        if owner == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if owner != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let mut product = self.products.setter(product_id);
        if product.businessOwner.get() != owner {
            return Err(OnClickContractError::ProductNotFound(ProductNotFound {}));
        }

        product.isActive.set(false);

        evm::log(ProductDeleted {
            owner,
            productId: product_id,
        });

        Ok(())
    }

    /// Get product by ID
    pub fn get_product(
        &self,
        product_id: U256,
    ) -> Result<(U256, Address, String, U256, String, bool, U256, U256), OnClickContractError> {
        let product = self.products.getter(product_id);

        if product.businessOwner.get() == Address::ZERO {
            return Err(OnClickContractError::ProductNotFound(ProductNotFound {}));
        }

        Ok((
            product.id.get(),
            product.businessOwner.get(),
            product.name.get_string(),
            product.price.get(),
            product.metadataHash.get_string(),
            product.isActive.get(),
            product.totalSold.get(),
            product.createdAt.get(),
        ))
    }

    // ==================== Platform Management ====================

    /// Set platform fee (admin only)
    pub fn set_platform_fee(&mut self, fee_percentage: U256) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        if sender != self.owner.get() {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        self.platformFee.set(fee_percentage);

        evm::log(PlatformFeeUpdated {
            newFee: fee_percentage,
        });

        Ok(())
    }

    /// Withdraw platform fees (admin only)
    pub fn withdraw_platform_fees(&mut self) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        if sender != self.owner.get() {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        let fees = self.platformFeesCollected.get();
        self.platformFeesCollected.set(U256::ZERO);

        self.vm().transfer_eth(sender, fees)?;

        evm::log(PlatformFeesWithdrawn { amount: fees });

        Ok(())
    }

    /// Get next ID (utility function)
    pub fn get_next_id(&self) -> U256 {
        self.pageId.get()
    }

    // ==================== Payment Intent Functions ====================

    /// Create a payment intent (payment link)
    pub fn create_payment_intent(
        &mut self,
        amount: U256,
        description: String,
        expires_in_seconds: U256,
        max_usages: U256,
    ) -> Result<FixedBytes<32>, OnClickContractError> {
        let sender = self.vm().msg_sender();
        let page = self.pages.getter(sender);

        if page.owner.get() == Address::ZERO {
            return Err(OnClickContractError::PageNotFound(PageNotFound {}));
        }

        if amount == U256::ZERO {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        let timestamp = self.vm().block_timestamp();
        if expires_in_seconds == U256::ZERO {
            return Err(OnClickContractError::InvalidExpiration(
                InvalidExpiration {},
            ));
        }

        // Generate unique intent ID
        let mut data = Vec::new();
        data.extend_from_slice(sender.as_slice());
        data.extend_from_slice(&timestamp.to_le_bytes());
        data.extend_from_slice(&amount.to_le_bytes::<32>());

        let intent_id = crypto::keccak(data);

        let expires_at = U256::from(timestamp) + expires_in_seconds;
        let handle = page.handle.get_string();

        // Store payment intent
        let mut intent = self.paymentIntents.setter(intent_id);
        intent.id.set(intent_id);
        intent.creator.set(sender);
        intent.handle.set_str(&handle);
        intent.amount.set(amount);
        intent.description.set_str(&description);
        intent.isActive.set(true);
        intent.createdAt.set(U256::from(timestamp));
        intent.expiresAt.set(expires_at);
        intent.usageCount.set(U256::ZERO);
        intent.maxUsages.set(max_usages);

        // Add to handle and user mappings
        self.handlePaymentIntents
            .setter(handle.clone())
            .push(intent_id);
        self.userPaymentIntents.setter(sender).push(intent_id);

        evm::log(PaymentIntentCreated {
            intentId: intent_id,
            creator: sender,
            handle: handle,
            amount,
        });

        Ok(intent_id)
    }

    /// Pay via payment intent
    #[payable]
    pub fn pay_intent(
        &mut self,
        intent_id: FixedBytes<32>,
        message_hash: String,
    ) -> Result<U256, OnClickContractError> {
        let amount = self.vm().msg_value();
        if amount == U256::ZERO {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        let sender = self.vm().msg_sender();
        let current_time = U256::from(self.vm().block_timestamp());

        let mut intent = self.paymentIntents.setter(intent_id);

        // Check if intent exists
        if intent.creator.get() == Address::ZERO {
            return Err(OnClickContractError::PaymentIntentNotFound(
                PaymentIntentNotFound {},
            ));
        }

        // Check if active
        if !intent.isActive.get() {
            return Err(OnClickContractError::PaymentIntentInactive(
                PaymentIntentInactive {},
            ));
        }

        // Check expiration
        let expires_at = intent.expiresAt.get();
        if current_time > expires_at {
            return Err(OnClickContractError::PaymentIntentExpired(
                PaymentIntentExpired {},
            ));
        }

        // Check max usages and get values before mutation
        let max_usages = intent.maxUsages.get();
        let expected_amount = intent.amount.get();
        let recipient = intent.creator.get();

        if max_usages != U256::ZERO {
            let usage_count = intent.usageCount.get();
            if usage_count >= max_usages {
                return Err(OnClickContractError::PaymentIntentMaxUsages(
                    PaymentIntentMaxUsages {},
                ));
            }
            intent.usageCount.set(usage_count + U256::from(1));
        }

        // Verify amount matches
        if amount != expected_amount {
            return Err(OnClickContractError::InvalidAmount(InvalidAmount {}));
        }

        // Get recipient page for role validation
        let page = self.pages.getter(recipient);
        if !page.isActive.get() {
            return Err(OnClickContractError::PageNotActive(PageNotActive {}));
        }
        let recipient_role = page.role.get();

        // Calculate platform fee
        let fee = (amount * self.platformFee.get()) / U256::from(10000);
        let net_amount = amount - fee;

        // Update page stats
        let mut page_mut = self.pages.setter(recipient);
        let raised = page_mut.raised.get();
        page_mut.raised.set(raised + net_amount);
        let supporters = page_mut.supporters.get();
        page_mut.supporters.set(supporters + U256::from(1));

        // Transfer funds
        self.vm().transfer_eth(recipient, net_amount)?;

        // Update platform fees
        let fees = self.platformFeesCollected.get();
        self.platformFeesCollected.set(fees + fee);

        // Create transaction
        let tx_id = self.transactionId.get();
        self.transactionId.set(tx_id + U256::from(1));
        let timestamp = U256::from(self.vm().block_timestamp());

        let mut transaction = self.transactions.setter(tx_id);
        transaction.id.set(tx_id);
        transaction.from.set(sender);
        transaction.to.set(recipient);
        transaction.amount.set(net_amount);
        transaction.messageHash.set_str(&message_hash);
        transaction.recipientRole.set(recipient_role);
        transaction.productId.set(U256::ZERO);
        transaction.timestamp.set(timestamp);
        transaction.isRefunded.set(false);

        // Emit events
        evm::log(PaymentIntentPaid {
            intentId: intent_id,
            payer: sender,
            amount: net_amount,
        });

        evm::log(DonationMade {
            from: sender,
            to: recipient,
            amount: net_amount,
            messageHash: message_hash,
        });

        Ok(tx_id)
    }

    /// Cancel a payment intent
    pub fn cancel_payment_intent(
        &mut self,
        intent_id: FixedBytes<32>,
    ) -> Result<(), OnClickContractError> {
        let sender = self.vm().msg_sender();
        let mut intent = self.paymentIntents.setter(intent_id);

        // Check if intent exists
        if intent.creator.get() == Address::ZERO {
            return Err(OnClickContractError::PaymentIntentNotFound(
                PaymentIntentNotFound {},
            ));
        }

        // Check ownership
        if intent.creator.get() != sender {
            return Err(OnClickContractError::NotPageOwner(NotPageOwner {}));
        }

        intent.isActive.set(false);

        evm::log(PaymentIntentCancelled {
            intentId: intent_id,
        });

        Ok(())
    }

    /// Get payment intent details
    pub fn get_payment_intent(
        &self,
        intent_id: FixedBytes<32>,
    ) -> Result<(Address, String, U256, String, bool, U256, U256, U256, U256), OnClickContractError>
    {
        let intent = self.paymentIntents.getter(intent_id);

        if intent.creator.get() == Address::ZERO {
            return Err(OnClickContractError::PaymentIntentNotFound(
                PaymentIntentNotFound {},
            ));
        }

        Ok((
            intent.creator.get(),
            intent.handle.get_string(),
            intent.amount.get(),
            intent.description.get_string(),
            intent.isActive.get(),
            intent.createdAt.get(),
            intent.expiresAt.get(),
            intent.usageCount.get(),
            intent.maxUsages.get(),
        ))
    }

    /// Get all payment intent IDs for a handle
    pub fn get_payment_intents_by_handle(&self, handle: String) -> Vec<FixedBytes<32>> {
        let intents = self.handlePaymentIntents.getter(handle);
        let len = intents.len();
        let mut result = Vec::new();

        for i in 0..len {
            result.push(intents.get(i).unwrap());
        }

        result
    }

    /// Get all payment intent IDs for an address
    pub fn get_payment_intents_by_address(&self, user: Address) -> Vec<FixedBytes<32>> {
        let intents = self.userPaymentIntents.getter(user);
        let len = intents.len();
        let mut result = Vec::new();

        for i in 0..len {
            result.push(intents.get(i).unwrap());
        }

        result
    }

    /// Get payment intent count for a handle
    pub fn get_payment_intent_count_by_handle(&self, handle: String) -> U256 {
        let intents = self.handlePaymentIntents.getter(handle);
        U256::from(intents.len())
    }

    /// Get payment intent count for an address
    pub fn get_payment_intent_count_by_address(&self, user: Address) -> U256 {
        let intents = self.userPaymentIntents.getter(user);
        U256::from(intents.len())
    }
}
