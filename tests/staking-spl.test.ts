import * as anchor from "@coral-xyz/anchor";

import { bn, parseGlobalState, parseStake } from "./utils/functions";
import {
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

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
  const randomWallet = anchor.web3.Keypair.generate();

  const [globalStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [SEEDS.GLOBAL_STATE_SEED],
    program.programId
  );

  const [stakePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [SEEDS.STAKE_SEED, randomWallet.publicKey.toBuffer()],
    program.programId
  );

  const depositEventListener = program.addEventListener("depositSplEvent", (event, slot) => {
    console.log("Deposit Event:", event, "at slot:", slot);
  });

  const claimEventListener = program.addEventListener("claimRewardsSplEvent", (event, slot) => {
    console.log("Claim Rewards Event:", event, "at slot:", slot);
  });

  const closeEventListener = program.addEventListener("closePositionSplEvent", (event, slot) => {
    console.log("Close Position Event:", event, "at slot:", slot);
  });

  const withdrawFeesEventListener = program.addEventListener("withdrawFeesEvent", (event, slot) => {
    console.log(
      "Withdraw Fees Event:",
      event,
      "at slot:",
      slot,
      "Amount:",
      event.amount.toNumber()
    );
  });

  it("Is initialized!", async () => {
    const rewardRate = bn(1_902_000_000); // 1_902_000 = 6% APY

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

    // airdrop to randomWallet
    const airdropSig = await connection.requestAirdrop(
      randomWallet.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSig);

    const walletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      randomMintAddress.publicKey,
      randomWallet.publicKey
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
        depositor: randomWallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        mint: randomMintAddress.publicKey,
      })
      .signers([randomWallet])
      .rpc();

    console.log("Deposit tx signature:", tx);

    const globalState = await program.account.globalState.fetch(globalStatePda);
    const stake = await program.account.stake.fetch(stakePda);

    console.log(parseGlobalState(globalState));
    console.log(parseStake(stake));

    const amountAfterFee = (amount.toNumber() * 98) / 100; // 2% fee
    expect(stake.amount.toNumber()).to.equal(amountAfterFee);
    expect(globalState.totalStaked.toNumber()).to.greaterThan(0);
  });

  it("Claim rewards", async () => {
    // wait 15 seconds to accumulate rewards
    await new Promise((resolve) => setTimeout(resolve, 15_000));

    const stakeBeforeClaim = await program.account.stake.fetch(stakePda);
    console.log(parseStake(stakeBeforeClaim));

    const tx = await program.methods
      .claimRewardsSpl()
      .accounts({
        authority: wallet.publicKey,
        claimer: randomWallet.publicKey,
        mint: randomMintAddress.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([randomWallet, wallet.payer])
      .rpc();

    console.log("Claim rewards tx signature:", tx);

    const stake = await program.account.stake.fetch(stakePda);
    console.log(parseStake(stake));

    expect(stake.unclaimedRewards.toNumber()).to.equal(0);
  });

  it("Close stake and claim rewards", async () => {
    const stakeBeforeClose = await program.account.stake.fetch(stakePda);
    console.log(parseStake(stakeBeforeClose));

    const tx = await program.methods
      .closePositionSpl()
      .accounts({
        authority: wallet.publicKey,
        closer: randomWallet.publicKey,
        mint: randomMintAddress.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([randomWallet, wallet.payer])
      .rpc();

    console.log("Close position tx signature:", tx);

    const globalState = await program.account.globalState.fetch(globalStatePda);
    const stake = await program.account.stake.fetchNullable(stakePda);

    console.log(parseGlobalState(globalState));
    console.log("Stake after close:", stake);

    expect(stake).to.be.null;
  });

  it("Withdraw fees", async () => {
    const tx = await program.methods
      .withdrawFees()
      .accounts({
        mint: randomMintAddress.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Withdraw fees tx signature:", tx);

    const globalState = await program.account.globalState.fetch(globalStatePda);
    console.log(parseGlobalState(globalState));

    expect(globalState.treasuryAmount.toNumber()).to.equal(0);
  });

  after(async () => {
    await program.removeEventListener(depositEventListener);
    await program.removeEventListener(claimEventListener);
    await program.removeEventListener(closeEventListener);
    await program.removeEventListener(withdrawFeesEventListener);
  });
});
