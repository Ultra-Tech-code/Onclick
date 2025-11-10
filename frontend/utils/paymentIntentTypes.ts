/**
 * Payment Intent Types for OnClick Platform
 * These types mirror the smart contract PaymentIntent structure
 */

export interface PaymentIntent {
    intentId: string; // bytes32 as hex string
    creator: string; // address
    handle: string;
    amount: string; // in wei as string to avoid precision loss
    description: string;
    isActive: boolean;
    createdAt: number; // Unix timestamp
    expiresAt: number; // Unix timestamp
    usageCount: number;
    maxUsages: number; // 0 = unlimited
}

export interface CreatePaymentIntentParams {
    amount: string; // in ETH (will be converted to wei)
    description: string;
    expiryDays: number;
    maxUsages: number; // 0 for unlimited
}

export interface PaymentIntentFormData {
    amount: string;
    description: string;
    expiryType: "hours" | "days" | "weeks";
    expiryValue: number;
    maxUsages: number;
    isUnlimited: boolean;
}

export interface PaymentLinkMetadata {
    intentId: string;
    shortUrl?: string;
    qrCodeUrl?: string;
    createdBy: string;
    shareableLink: string;
}

export enum PaymentIntentStatus {
    ACTIVE = "active",
    EXPIRED = "expired",
    CANCELLED = "cancelled",
    MAX_USAGE_REACHED = "max_usage_reached",
}

export interface PaymentIntentStats {
    totalCreated: number;
    totalActive: number;
    totalRevenue: string; // in ETH
    totalPayments: number;
    averageAmount: string; // in ETH
}

