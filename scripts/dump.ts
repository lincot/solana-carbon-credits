import * as anchor from "@project-serum/anchor";
import * as bs58 from "bs58";
import * as bs64 from "base64-js";
import { writeFile } from "fs";
import { execSync } from "child_process";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const wallet = (provider.wallet as NodeWallet).payer;

function dumpMint(
  mintAddress: string,
  tokenName: string,
  mintAuthority: string
) {
  const mintPath = `tests/accounts/${tokenName}.json`;
  execSync(
    `mkdir -p tests/accounts && solana account ${mintAddress} --output json -o ${mintPath} -um >/dev/null`
  );
  const mint = require(`../${mintPath}`);

  let data = bs64.toByteArray(mint["account"]["data"][0]);
  const authority = bs58.decode(mintAuthority);
  data = Buffer.concat([data.slice(0, 4), authority, data.slice(4 + 32)]);
  mint["account"]["data"][0] = bs64.fromByteArray(data);

  writeFile(mintPath, JSON.stringify(mint), "utf8", (err: any) => {
    if (err) {
      console.log(err);
      process.exit(1);
    }
  });
}

dumpMint(
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "USDC",
  wallet.publicKey.toString()
);
