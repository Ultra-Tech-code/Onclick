import { pinFileWithPinata, pinJsonWithPinata } from "./pinta";
import type {
    PageMetadata,
    ProductMetadata,
    CampaignMetadata,
    TransactionMessage,
    ProfileExtended,
    IPFSHash,
} from "./ipfsTypes";

/**
 * Upload page metadata to IPFS
 * @param metadata - Page metadata object
 * @param bannerFile - Optional banner image file
 * @param avatarFile - Optional avatar image file
 * @returns IPFS hash (ipfs://...)
 */
export async function uploadPageMetadata(
    metadata: Omit<PageMetadata, "bannerUrl" | "avatarUrl">,
    bannerFile?: File,
    avatarFile?: File
): Promise<IPFSHash> {
    // Upload images if provided
    let bannerUrl = "";
    let avatarUrl = "";

    if (bannerFile) {
        bannerUrl = await pinFileWithPinata(bannerFile);
    }
    if (avatarFile) {
        avatarUrl = await pinFileWithPinata(avatarFile);
    }

    // Build complete metadata
    const pageMetadata: PageMetadata = {
        ...metadata,
        bannerUrl,
        avatarUrl,
    };

    // Upload metadata JSON to IPFS
    const ipfsHash = await pinJsonWithPinata(pageMetadata);
    return ipfsHash;
}

/**
 * Upload product metadata to IPFS
 * @param metadata - Product metadata object
 * @param imageFile - Main product image file
 * @param additionalImageFiles - Optional additional image files
 * @returns IPFS hash (ipfs://...)
 */
export async function uploadProductMetadata(
    metadata: Omit<ProductMetadata, "imageUrl" | "additionalImages">,
    imageFile: File,
    additionalImageFiles?: File[]
): Promise<IPFSHash> {
    // Upload main image
    const imageUrl = await pinFileWithPinata(imageFile);

    // Upload additional images if provided
    const additionalImages: string[] = [];
    if (additionalImageFiles && additionalImageFiles.length > 0) {
        for (const file of additionalImageFiles) {
            const url = await pinFileWithPinata(file);
            additionalImages.push(url);
        }
    }

    // Build complete metadata
    const productMetadata: ProductMetadata = {
        imageUrl,
        additionalImages: additionalImages.length > 0 ? additionalImages : undefined,
        ...metadata,
    };

    // Upload metadata JSON to IPFS
    const ipfsHash = await pinJsonWithPinata(productMetadata);
    return ipfsHash;
}

/**
 * Upload campaign metadata to IPFS
 * @param metadata - Campaign metadata object
 * @param bannerFile - Optional campaign banner image file
 * @param videoFile - Optional campaign video file
 * @param milestoneImages - Optional milestone image files (matching milestone order)
 * @returns IPFS hash (ipfs://...)
 */
export async function uploadCampaignMetadata(
    metadata: Omit<CampaignMetadata, "bannerUrl" | "videoUrl">,
    bannerFile?: File,
    videoFile?: File,
    milestoneImages?: File[]
): Promise<IPFSHash> {
    // Upload banner if provided
    let bannerUrl: string | undefined;
    if (bannerFile) {
        bannerUrl = await pinFileWithPinata(bannerFile);
    }

    // Upload video if provided
    let videoUrl: string | undefined;
    if (videoFile) {
        videoUrl = await pinFileWithPinata(videoFile);
    }

    // Upload milestone images if provided
    const milestones = metadata.milestones ? [...metadata.milestones] : undefined;
    if (milestoneImages && milestones) {
        for (let i = 0; i < Math.min(milestoneImages.length, milestones.length); i++) {
            if (milestoneImages[i]) {
                milestones[i] = {
                    ...milestones[i],
                    imageUrl: await pinFileWithPinata(milestoneImages[i]),
                };
            }
        }
    }

    // Build complete metadata
    const campaignMetadata: CampaignMetadata = {
        ...metadata,
        milestones,
        ...(bannerUrl && { bannerUrl }),
        ...(videoUrl && { videoUrl }),
    };

    // Upload metadata JSON to IPFS
    const ipfsHash = await pinJsonWithPinata(campaignMetadata);
    return ipfsHash;
}

/**
 * Upload transaction message to IPFS
 * @param message - Transaction message object
 * @returns IPFS hash (ipfs://...) or empty string if no message
 */
export async function uploadTransactionMessage(
    message: TransactionMessage
): Promise<IPFSHash | ""> {
    // If message is empty, return empty string
    if (!message.message || message.message.trim() === "") {
        return "";
    }

    // Upload message JSON to IPFS
    const ipfsHash = await pinJsonWithPinata(message);
    return ipfsHash;
}

/**
 * Upload profile extended data to IPFS
 * @param profileData - Profile extended data object
 * @param portfolioFiles - Optional portfolio image files
 * @returns IPFS hash (ipfs://...)
 */
export async function uploadProfileExtended(
    profileData: Omit<ProfileExtended, "portfolio">,
    portfolioFiles?: File[]
): Promise<IPFSHash> {
    // Upload portfolio images if provided
    const portfolio: string[] = [];
    if (portfolioFiles && portfolioFiles.length > 0) {
        for (const file of portfolioFiles) {
            const url = await pinFileWithPinata(file);
            portfolio.push(url);
        }
    }

    // Build complete profile data
    const profileExtended: ProfileExtended = {
        portfolio: portfolio.length > 0 ? portfolio : undefined,
        ...profileData,
    };

    // Upload metadata JSON to IPFS
    const ipfsHash = await pinJsonWithPinata(profileExtended);
    return ipfsHash;
}

/**
 * Extract IPFS hash from IPFS URL
 * @param ipfsUrl - IPFS URL (ipfs://QmXXX...)
 * @returns Hash string (QmXXX...)
 */
export function extractIpfsHash(ipfsUrl: string): string {
    return ipfsUrl.replace("ipfs://", "");
}

/**
 * Convert IPFS hash to gateway URL
 * @param ipfsHash - IPFS hash or URL
 * @param gateway - IPFS gateway (default: https://ipfs.io/ipfs/)
 * @returns Gateway URL
 */
export function getIpfsGatewayUrl(
    ipfsHash: string,
    gateway: string = "https://ipfs.io/ipfs/"
): string {
    const hash = extractIpfsHash(ipfsHash);
    return `${gateway}${hash}`;
}

