import * as anchor from "@project-serum/anchor";
import { Program, AnchorProvider } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Carbon } from "../target/types/carbon";
import { airdrop } from "./utils";
import * as token from "@solana/spl-token";
import { findOrCreateATA } from "./token";

export const USDC_MINT = new PublicKey(
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
);

export type CNFTTier =
  | { platinum: {} }
  | { gold: {} }
  | { silver: {} }
  | { bronze: {} };

export class Context {
  provider: AnchorProvider;

  program: Program<Carbon>;

  authority: Keypair;

  user: Keypair;

  programAsSigner: PublicKey;
  ccMint: PublicKey;
  ccReserve: PublicKey;

  whitelistMint: PublicKey;

  payer: Keypair;

  constructor() {
    this.provider = anchor.AnchorProvider.env();
    anchor.setProvider(this.provider);
    this.program = anchor.workspace.Carbon;
    this.payer = (this.provider.wallet as NodeWallet).payer;

    this.authority = new Keypair();
    this.user = new Keypair();

    this.programAsSigner = findProgramAddressSync(
      [Buffer.from("program_as_signer")],
      this.program.programId
    )[0];
    this.ccMint = findProgramAddressSync(
      [Buffer.from("cc_mint")],
      this.program.programId
    )[0];
    this.ccReserve = findProgramAddressSync(
      [Buffer.from("cc_reserve")],
      this.program.programId
    )[0];
  }

  async setup(): Promise<void> {
    await airdrop(this, [this.authority.publicKey, this.user.publicKey]);

    await token.mintTo(
      this.provider.connection,
      this.payer,
      USDC_MINT,
      await findOrCreateATA(this, USDC_MINT, this.user.publicKey),
      this.payer,
      1_000_000
    );

    this.whitelistMint = await token.createMint(
      this.provider.connection,
      this.payer,
      this.payer.publicKey,
      this.payer.publicKey,
      0
    );
    await token.mintTo(
      this.provider.connection,
      this.payer,
      this.whitelistMint,
      await findOrCreateATA(this, this.whitelistMint, this.user.publicKey),
      this.payer,
      1
    );
  }
}
