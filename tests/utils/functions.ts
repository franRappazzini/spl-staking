import { BN } from "bn.js";

export function bn(n: number) {
  return new BN(n);
}

export function parseGlobalState(gs: any) {
  return {
    authority: gs.authority.toBase58(),
    mint: gs.mint.toBase58(),
    vault: gs.vault.toBase58(),
    totalStaked: gs.totalStaked.toNumber(),
    treasuryAmount: gs.treasuryAmount.toNumber(),
    accRewardPerShare: gs.accRewardPerShare.toNumber(),
    lastUpdateTime: gs.lastUpdateTime.toNumber(),
    rewardRate: gs.rewardRate.toNumber(),
    bump: gs.bump,
  };
}

export function parseStake(stake: any) {
  return {
    owner: stake.owner.toBase58(),
    amount: stake.amount.toNumber(),
    startTime: stake.startTime.toNumber(),
    lastUpdateTime: stake.lastUpdateTime.toNumber(),
    rewardDebt: stake.rewardDebt.toNumber(),
    unclaimedRewards: stake.unclaimedRewards.toNumber(),
    bump: stake.bump,
  };
}
