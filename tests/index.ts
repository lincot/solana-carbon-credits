import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { Context } from "./ctx";
import {
  airdropCC,
  burnCC,
  initialize,
  createTierCollection,
  mintCC,
  mintCNFT,
  whitelist,
} from "./api";
import { PublicKey } from "@solana/web3.js";

chai.use(chaiAsPromised);

const ctx = new Context();

before(async () => {
  await ctx.setup();
});

describe("Carbon", () => {
  it("Initialize", async () => {
    await initialize(ctx);
  });

  it("CreateTierEdition", async () => {
    await createTierCollection(ctx, { platinum: {} }, "");
    await createTierCollection(ctx, { gold: {} }, "");
    await createTierCollection(ctx, { silver: {} }, "");
    await createTierCollection(ctx, { bronze: {} }, "");
  });

  it("Whitelist", async () => {
    await whitelist(ctx, ctx.user.publicKey);
  });

  let cnftMint: PublicKey;

  it("MintCNFT", async () => {
    ({ cnftMint } = await mintCNFT(ctx, ctx.user, { gold: {} }));
  });

  it("MintCC", async () => {
    await mintCC(ctx, 500, "https://verra.org/...");
  });

  it("AirdropCC", async () => {
    await airdropCC(ctx, ctx.user.publicKey, cnftMint);
  });

  it("BurnCC", async () => {
    await burnCC(ctx, ctx.user, 50);
  });
});
