import * as anchor from "@coral-xyz/anchor";

import { bn, parseGlobalState } from "./utils/functions";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

import { Program } from "@coral-xyz/anchor";
import SEEDS from "./utils/seeds";
import { StakingSpl } from "../target/types/staking_spl";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { expect } from "chai";
import { publicKey } from "./../node_modules/@solana/buffer-layout-utils/src/web3";

describe("staking-spl", () => {
  const provider = anchor.AnchorProvider.env();
  const { connection, wallet } = provider;

  anchor.setProvider(provider);

  const program = anchor.workspace.stakingSpl as Program<StakingSpl>;

  const randomMintAddress = anchor.web3.Keypair.generate();

  const [globalStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [SEEDS.GLOBAL_STATE_SEED],
    program.programId
  );

  it("Is initialized!", async () => {
    const rewardRate = bn(1_902_000); // 6% APY

    console.log("Mint address:", randomMintAddress.publicKey.toBase58());

    const tx = await program.methods
      .initialize(rewardRate)
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        mint: randomMintAddress.publicKey,
      })
      .signers([randomMintAddress])
      .rpc();

    console.log("Initialize tx signature:", tx);

    const walletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      randomMintAddress.publicKey,
      wallet.publicKey
    );

    await mintTo(
      connection,
      wallet.payer,
      randomMintAddress.publicKey,
      walletAta.address,
      wallet.payer,
      1_000 * anchor.web3.LAMPORTS_PER_SOL
    );

    const globalState = await program.account.globalState.fetch(globalStatePda);

    expect(globalState.mint.toBase58()).to.equal(randomMintAddress.publicKey.toBase58());
  });

  it("Deposit SPL tokens", async () => {
    const amount = bn(500 * anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .depositSpl(amount)
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        mint: randomMintAddress.publicKey,
      })
      .rpc();

    console.log("Deposit tx signature:", tx);

    const globalState = await program.account.globalState.fetch(globalStatePda);

    console.log(parseGlobalState(globalState));

    expect(globalState.totalStaked.toNumber()).to.greaterThan(0);
  });
});
