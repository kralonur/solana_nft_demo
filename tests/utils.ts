import * as anchor from "@project-serum/anchor";

// Functions

async function getPDAPublicKey(seeds: Array<Buffer | Uint8Array>, programId: anchor.web3.PublicKey) {
  return (await getPDA(seeds, programId))[0];
}

function getPDA(seeds: Array<Buffer | Uint8Array>, programId: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
}

// Types

type TInitialized = { data: Number; label: string };
type TNFTMinted = { nftNum: Number };

export { getPDAPublicKey, TInitialized, TNFTMinted };
