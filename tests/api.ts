import { BN } from "@project-serum/anchor";
import {
  SystemProgram,
  Keypair,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
  TransactionSignature,
} from "@solana/web3.js";
import { CnftTier, Context } from "./ctx";
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import {
  findAssociatedTokenAccountPda,
  findMasterEditionV2Pda,
  findMetadataPda,
} from "@metaplex-foundation/js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { findOrCreateATA } from "./token";
import { USDC_MINT } from "./utils";

export async function initialize(ctx: Context): Promise<TransactionSignature> {
  return await ctx.program.methods
    .initialize()
    .accounts({
      programState: ctx.programState,
      authority: ctx.authority.publicKey,
      ccMint: ctx.ccMint,
      ccReserve: ctx.ccReserve,
      whitelistMint: ctx.whitelistMint,
      rent: SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.authority])
    .rpc();
}

export function findTierCollectionMint(
  ctx: Context,
  tier: CnftTier
): PublicKey {
  let tierAsNumber = 0;
  if (tier.hasOwnProperty("gold")) {
    tierAsNumber = 1;
  } else if (tier.hasOwnProperty("silver")) {
    tierAsNumber = 2;
  } else if (tier.hasOwnProperty("bronze")) {
    tierAsNumber = 3;
  }

  return findProgramAddressSync(
    [Buffer.from("tier_collection_mint"), Buffer.from([tierAsNumber])],
    ctx.program.programId
  )[0];
}

export function findCnftData(ctx: Context, mint: PublicKey): PublicKey {
  return findProgramAddressSync(
    [Buffer.from("cnft_data"), mint.toBuffer()],
    ctx.program.programId
  )[0];
}

export async function createTierCollection(
  ctx: Context,
  tier: CnftTier
): Promise<TransactionSignature> {
  const mint = findTierCollectionMint(ctx, tier);

  return await ctx.program.methods
    .createTierCollection(tier)
    .accounts({
      programState: ctx.programState,
      tokenAccount: findAssociatedTokenAccountPda(mint, ctx.programState),
      authority: ctx.authority.publicKey,
      metadata: findMetadataPda(mint),
      mint,
      edition: findMasterEditionV2Pda(mint),
      rent: SYSVAR_RENT_PUBKEY,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.authority])
    .rpc();
}

export async function whitelist(
  ctx: Context,
  user: PublicKey
): Promise<TransactionSignature> {
  return await ctx.program.methods
    .whitelist()
    .accounts({
      programState: ctx.programState,
      authority: ctx.authority.publicKey,
      whitelistMint: ctx.whitelistMint,
      tokenAccount: await findOrCreateATA(ctx, ctx.whitelistMint, user),
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([ctx.authority])
    .rpc();
}

export async function mintCnft(
  ctx: Context,
  authority: Keypair,
  tier: CnftTier
): Promise<{
  transactionSignature: TransactionSignature;
  cnftMint: PublicKey;
}> {
  const mint = new Keypair();
  const collectionMint = findTierCollectionMint(ctx, tier);

  return {
    transactionSignature: await ctx.program.methods
      .mintCnft(tier)
      .accounts({
        programState: ctx.programState,
        authority: authority.publicKey,
        authorityWhitelist: await findOrCreateATA(
          ctx,
          ctx.whitelistMint,
          authority.publicKey
        ),
        authorityUsdc: await findOrCreateATA(
          ctx,
          USDC_MINT,
          authority.publicKey
        ),
        platformUsdc: await findOrCreateATA(
          ctx,
          USDC_MINT,
          ctx.authority.publicKey
        ),
        mint: mint.publicKey,
        metadata: findMetadataPda(mint.publicKey),
        edition: findMasterEditionV2Pda(mint.publicKey),
        tokenAccount: findAssociatedTokenAccountPda(
          mint.publicKey,
          authority.publicKey
        ),
        collectionMint,
        collectionMetadata: findMetadataPda(collectionMint),
        collectionEdition: findMasterEditionV2Pda(collectionMint),
        cnftData: findCnftData(ctx, mint.publicKey),
        rent: SYSVAR_RENT_PUBKEY,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, mint])
      .rpc(),
    cnftMint: mint.publicKey,
  };
}

export async function mintCC(
  ctx: Context,
  amount: number | BN,
  registryBatchUri: string
): Promise<TransactionSignature> {
  return await ctx.program.methods
    .mintCc(new BN(amount), registryBatchUri)
    .accounts({
      programState: ctx.programState,
      authority: ctx.authority.publicKey,
      ccMint: ctx.ccMint,
      ccReserve: ctx.ccReserve,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([ctx.authority])
    .rpc();
}

export async function airdropCC(
  ctx: Context,
  authority: PublicKey,
  cnftMint: PublicKey
): Promise<TransactionSignature> {
  return await ctx.program.methods
    .airdropCc()
    .accounts({
      programState: ctx.programState,
      cnftAccount: await findOrCreateATA(ctx, cnftMint, authority),
      cnftData: findCnftData(ctx, cnftMint),
      ccReserve: ctx.ccReserve,
      ccAccount: await findOrCreateATA(ctx, ctx.ccMint, authority),
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
}

export async function burnCC(
  ctx: Context,
  authority: Keypair,
  amount: number | BN
): Promise<TransactionSignature> {
  return await ctx.program.methods
    .burnCc(new BN(amount))
    .accounts({
      authority: authority.publicKey,
      ccMint: ctx.ccMint,
      ccAccount: await findOrCreateATA(ctx, ctx.ccMint, authority.publicKey),
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([authority])
    .rpc();
}
