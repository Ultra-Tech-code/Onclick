/**
 * TypeScript interfaces for IPFS metadata structures
 * These match the structures defined in READMEcontract.md
 */

// ==================== Page Metadata ====================
export interface SocialLinks {
    twitter?: string;
    instagram?: string;
    website?: string;
    youtube?: string;
    tiktok?: string;
    discord?: string;
    [key: string]: string | undefined;
}

export interface PageMetadata {
    name: string;
    description: string;
    bannerUrl: string; // ipfs://...
    avatarUrl: string; // ipfs://...
    theme: string; // hex color
    socialLinks?: SocialLinks;
    customFields?: Record<string, any>;
}

// ==================== Product Metadata ====================
export interface ProductMetadata {
    name: string;
    description: string;
    imageUrl: string; // ipfs://...
    additionalImages?: string[]; // ipfs://...
    category?: string;
    tags?: string[];
    specifications?: Record<string, any>;
}

// ==================== Campaign Metadata ====================
export interface CampaignMilestoneMetadata {
    title: string;
    description: string;
    imageUrl?: string; // ipfs://...
}

export interface CampaignUpdate {
    title: string;
    content: string;
    imageUrl?: string; // ipfs://...
    timestamp: number;
}

export interface CampaignMetadata {
    title: string;
    story: string;
    bannerUrl?: string; // ipfs://...
    videoUrl?: string; // ipfs://...
    milestones?: CampaignMilestoneMetadata[];
    updates?: CampaignUpdate[];
}

// ==================== Transaction Message ====================
export interface TransactionMessage {
    message: string;
    supporterName?: string;
    isPublic: boolean;
}

// ==================== Profile Extended Data ====================
export interface ProfileExtended {
    bio?: string;
    location?: string;
    skills?: string[];
    portfolio?: string[]; // ipfs://...
    achievements?: Array<{
        title: string;
        description: string;
        date?: string;
        imageUrl?: string; // ipfs://...
    }>;
}

// ==================== Helper Types ====================
export type IPFSHash = string; // ipfs://QmXXX...
export type IPFSMetadataType =
    | 'page'
    | 'product'
    | 'campaign'
    | 'transaction-message'
    | 'profile-extended';

