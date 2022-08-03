import * as chai from "chai";
import * as token from "@solana/spl-token";
import chaiAsPromised from "chai-as-promised";
import { Context } from "./ctx";
import {
  airdropCC,
  burnCC,
  initialize,
  createTierCollection,
  mintCC,
  mintCnft,
  whitelist,
  findTierCollectionMint,
  findCnftData,
} from "./api";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { getMint } from "@solana/spl-token";

chai.use(chaiAsPromised);

const ctx = new Context();

before(async () => {
  await ctx.setup();
});

describe("Carbon", () => {
  it("Initialize", async () => {
    await initialize(ctx);

    const programState = await ctx.program.account.programState.fetch(
      ctx.programState
    );
    expect(programState.bump).to.gt(200);
    expect(programState.authority).to.eql(ctx.authority.publicKey);

    const ccMint = await getMint(ctx.provider.connection, ctx.ccMint);
    expect(ccMint.mintAuthority).to.eql(ctx.programState);
    expect(ccMint.decimals).to.eql(ctx.ccDecimals);

    const ccReserve = await token.getAccount(
      ctx.provider.connection,
      ctx.ccReserve
    );
    expect(ccReserve.owner).to.eql(ctx.programState);

    const whitelistMint = await getMint(
      ctx.provider.connection,
      ctx.whitelistMint
    );
    expect(whitelistMint.mintAuthority).to.eql(ctx.programState);
    expect(whitelistMint.freezeAuthority).to.eql(ctx.programState);
    expect(whitelistMint.decimals).to.eql(0);
  });

  it("CreateTierEdition", async () => {
    await Promise.all(
      [{ platinum: {} }, { gold: {} }, { silver: {} }, { bronze: {} }].map(
        (tier) => createTierCollection(ctx, tier)
      )
    );

    const silverCollection = await ctx.metaplex
      .nfts()
      .findByMint(findTierCollectionMint(ctx, { silver: {} }))
      .run();
    expect(silverCollection.updateAuthorityAddress).to.eql(ctx.programState);
  });

  it("Whitelist", async () => {
    await whitelist(ctx, ctx.user1.publicKey);

    const tokenAccount = await token.getOrCreateAssociatedTokenAccount(
      ctx.provider.connection,
      ctx.payer,
      ctx.whitelistMint,
      ctx.user1.publicKey
    );
    expect(tokenAccount.amount).to.eql(BigInt(1));
    expect(tokenAccount.isFrozen).to.eql(true);
  });

  let cnftMint: PublicKey;

  it("MintCnft", async () => {
    ({ cnftMint } = await mintCnft(ctx, ctx.user1, { gold: {} }));

    const cnft = await ctx.metaplex.nfts().findByMint(cnftMint).run();
    expect(cnft.updateAuthorityAddress).to.eql(ctx.programState);
    expect(cnft.collection.key).to.eql(
      findTierCollectionMint(ctx, { gold: {} })
    );
    expect(cnft.collection.verified).to.eql(true);

    const cnftData = await ctx.program.account.cnftData.fetch(
      findCnftData(ctx, cnftMint)
    );
    expect(cnftData.creationTimestamp).to.be.within(
      +new Date() / 1000 - 7,
      +new Date() / 1000
    );
    expect(cnftData.creditsPerYear).to.eql(100);
  });

  it("MintCC", async () => {
    await mintCC(ctx, 500, "https://verra.org/...");

    const ccReserve = await token.getAccount(
      ctx.provider.connection,
      ctx.ccReserve
    );
    expect(ccReserve.amount).to.eql(BigInt(500_000_000_000));
  });

  it("AirdropCC", async () => {
    await airdropCC(ctx, ctx.user1.publicKey, cnftMint);

    let ccAccount = await token.getOrCreateAssociatedTokenAccount(
      ctx.provider.connection,
      ctx.payer,
      ctx.ccMint,
      ctx.user1.publicKey
    );
    expect(ccAccount.amount).to.eql(BigInt(100_000_000_000));

    let cnftData = await ctx.program.account.cnftData.fetch(
      findCnftData(ctx, cnftMint)
    );
    expect(cnftData.airdropsClaimed).to.eql(1);

    await airdropCC(ctx, ctx.user1.publicKey, cnftMint);

    ccAccount = await token.getOrCreateAssociatedTokenAccount(
      ctx.provider.connection,
      ctx.payer,
      ctx.ccMint,
      ctx.user1.publicKey
    );
    expect(ccAccount.amount).to.eql(BigInt(100_000_000_000));

    cnftData = await ctx.program.account.cnftData.fetch(
      findCnftData(ctx, cnftMint)
    );
    expect(cnftData.airdropsClaimed).to.eql(1);
  });

  it("BurnCC", async () => {
    await burnCC(ctx, ctx.user1, 50);

    let ccAccount = await token.getOrCreateAssociatedTokenAccount(
      ctx.provider.connection,
      ctx.payer,
      ctx.ccMint,
      ctx.user1.publicKey
    );
    expect(ccAccount.amount).to.eql(BigInt(50_000_000_000));
  });
});
