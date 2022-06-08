import * as anchor from "@project-serum/anchor";

// Functions

async function getPDAPublicKey(seeds: Array<Buffer | Uint8Array>, programId: anchor.web3.PublicKey) {
  return (await getPDA(seeds, programId))[0];
}

function getPDA(seeds: Array<Buffer | Uint8Array>, programId: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
}

// Types

type TFinalized = { authority: string };
type TInitialized = { fee: Number };
type TNFTMinted = { nftNum: Number };
type TFeeUpdated = { fee: Number };
type TWithdrawn = { amount: Number; authority: string };

export { getPDAPublicKey, TFinalized, TInitialized, TNFTMinted, TFeeUpdated, TWithdrawn };
