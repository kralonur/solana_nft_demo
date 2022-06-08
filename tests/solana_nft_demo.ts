import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  getAssociatedTokenAddress,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token"; // IGNORE THESE ERRORS IF ANY
import { SolanaNftDemo } from "../target/types/solana_nft_demo";
import * as utils from "./utils";
const { SystemProgram } = anchor.web3;

describe("solana_nft_demo", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as Wallet;

  const program = anchor.workspace.SolanaNftDemo as Program<SolanaNftDemo>;

  const contractDataPublic = utils.getPDAPublicKey([Buffer.from("contractdata")], program.programId);

  const treasuryDataPublic = utils.getPDAPublicKey([Buffer.from("treasury")], program.programId);

  const getUserData = async (mintAuthority: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return utils.getPDAPublicKey([Buffer.from("userdata"), mintAuthority.toBuffer()], program.programId);
  };

  const handleFinalizedEvent = (ev: utils.TFinalized) =>
    console.log(`${program.idl.events[0].name} ==>`, {
      authority: ev.authority.toString(),
    });
  const handleInitializedEvent = (ev: utils.TInitialized) =>
    console.log(`${program.idl.events[1].name} ==>`, {
      fee: ev.fee.toString(),
    });
  const handleNFTMintedEvent = (ev: utils.TNFTMinted) =>
    console.log(`${program.idl.events[2].name} ==>`, {
      nftNum: ev.nftNum.toString(),
    });
  const handleFeeUpdatedEvent = (ev: utils.TFeeUpdated) =>
    console.log(`${program.idl.events[3].name} ==>`, {
      fee: ev.fee.toString(),
    });
  const handleWithdrawnEvent = (ev: utils.TWithdrawn) =>
    console.log(`${program.idl.events[4].name} ==>`, {
      amount: ev.amount.toString(),
      authority: ev.authority.toString(),
    });

  const finalizedListener = program.addEventListener(program.idl.events[0].name, handleFinalizedEvent);
  const initializedListener = program.addEventListener(program.idl.events[1].name, handleInitializedEvent);
  const nftMintedListener = program.addEventListener(program.idl.events[2].name, handleNFTMintedEvent);
  const feeUpdatedListener = program.addEventListener(program.idl.events[3].name, handleFeeUpdatedEvent);
  const withdrawnListener = program.addEventListener(program.idl.events[4].name, handleWithdrawnEvent);

  it("Initialize", async () => {
    console.log("contractDataPublic address ", (await contractDataPublic).toBase58());
    console.log("treasuryDataPublic address ", (await treasuryDataPublic).toBase58());

    const tx = await program.methods
      .initialize(new anchor.BN(5555))
      .accounts({
        contractData: await contractDataPublic,
        treasury: await treasuryDataPublic,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    console.log(await program.account.contractData.all());
  });

  it("Update fee", async () => {
    const tx = await program.methods
      .updateFee(new anchor.BN(123456789))
      .accounts({
        contractData: await contractDataPublic,
        authority: wallet.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    console.log(await program.account.contractData.all());
  });

  it("Mint!", async () => {
    // Add your test here.

    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
    const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
      return utils.getPDAPublicKey(
        [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
        TOKEN_METADATA_PROGRAM_ID,
      );
    };

    const getMasterEdition = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
      return utils.getPDAPublicKey(
        [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer(), Buffer.from("edition")],
        TOKEN_METADATA_PROGRAM_ID,
      );
    };

    // mint x times
    for (let i = 0; i < 3; i++) {
      const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
      const NftTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey, wallet.publicKey);
      console.log("NFT Account: ", NftTokenAccount.toBase58());

      const mint_tx = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: wallet.publicKey,
          newAccountPubkey: mintKey.publicKey,
          space: MINT_SIZE,
          programId: TOKEN_PROGRAM_ID,
          lamports,
        }),
        createInitializeMintInstruction(mintKey.publicKey, 0, wallet.publicKey, wallet.publicKey),
        createAssociatedTokenAccountInstruction(wallet.publicKey, NftTokenAccount, wallet.publicKey, mintKey.publicKey),
      );

      const res = await program.provider.sendAndConfirm(mint_tx, [mintKey]);
      console.log(await program.provider.connection.getParsedAccountInfo(mintKey.publicKey));

      console.log("Account: ", res);
      console.log("Mint key: ", mintKey.publicKey.toString());
      console.log("User: ", wallet.publicKey.toString());

      const metadataAddress = await getMetadata(mintKey.publicKey);
      const masterEdition = await getMasterEdition(mintKey.publicKey);

      console.log("Metadata address: ", metadataAddress.toBase58());
      console.log("MasterEdition: ", masterEdition.toBase58());

      const tx = await program.methods
        .mintNft(mintKey.publicKey, "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA", "Deez NUTZZZZ")
        .accounts({
          mintAuthority: wallet.publicKey,
          mint: mintKey.publicKey,
          tokenAccount: NftTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          metadata: metadataAddress,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          masterEdition: masterEdition,
          contractData: await contractDataPublic,
          userData: await getUserData(wallet.publicKey),
          treasury: await treasuryDataPublic,
        })
        .rpc();
      console.log("Your transaction signature", tx);
      console.log(await program.account.userData.all());
    }
  });

  it("Withdraw", async () => {
    const tx = await program.methods
      .withdraw(new anchor.BN(12345678))
      .accounts({
        contractData: await contractDataPublic,
        treasury: await treasuryDataPublic,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Finalize", async () => {
    const tx = await program.methods
      .finalize()
      .accounts({
        contractData: await contractDataPublic,
        userData: await getUserData(wallet.publicKey),
        treasury: await treasuryDataPublic,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Remove event listeners", async function () {
    program.removeEventListener(finalizedListener);
    program.removeEventListener(initializedListener);
    program.removeEventListener(nftMintedListener);
    program.removeEventListener(feeUpdatedListener);
    program.removeEventListener(withdrawnListener);
  });
});
