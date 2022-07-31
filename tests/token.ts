import { PublicKey } from "@solana/web3.js";
import { Context } from "./ctx";
import * as token from "@solana/spl-token";

export async function findOrCreateATA(
  ctx: Context,
  mint: PublicKey,
  owner: PublicKey
): Promise<PublicKey> {
  return (
    await token.getOrCreateAssociatedTokenAccount(
      ctx.provider.connection,
      ctx.payer,
      mint,
      owner,
      true
    )
  ).address;
}
